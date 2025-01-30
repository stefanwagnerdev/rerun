#include <array>
#include <filesystem>
#include <optional>
#include <vector>

#include <arrow/buffer.h>
#include <catch2/catch_test_macros.hpp>
#include <catch2/generators/catch_generators.hpp>
#include <rerun.hpp>

#include <rerun/c/rerun.h>

#include "error_check.hpp"

namespace fs = std::filesystem;

#define TEST_TAG "[recording_stream]"

struct BadComponent {};

template <>
struct rerun::Loggable<BadComponent> {
    static constexpr rerun::ComponentDescriptor Descriptor = "bad!";
    static rerun::Error error;

    static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
        return rerun::Loggable<rerun::components::Position2D>::arrow_datatype();
    }

    static rerun::Result<std::shared_ptr<arrow::Array>> to_arrow(const BadComponent*, size_t) {
        return error;
    }
};

rerun::Error rerun::Loggable<BadComponent>::error =
    rerun::Error(rerun::ErrorCode::Unknown, "BadComponent");

struct BadArchetype {
    size_t num_instances() const {
        return 1;
    }
};

namespace rerun {
    template <>
    struct AsComponents<BadArchetype> {
        static rerun::Result<std::vector<rerun::ComponentBatch>> serialize(const BadArchetype&) {
            return Loggable<BadComponent>::error;
        }
    };
} // namespace rerun

namespace rerun {
    std::ostream& operator<<(std::ostream& os, StoreKind kind) {
        switch (kind) {
            case rerun::StoreKind::Recording:
                os << "StoreKind::Recording";
                break;
            case rerun::StoreKind::Blueprint:
                os << "StoreKind::Blueprint";
                break;
            default:
                FAIL("Unknown StoreKind");
                break;
        }
        return os;
    }
} // namespace rerun

SCENARIO("RecordingStream can be created, destroyed and lists correct properties", TEST_TAG) {
    const auto kind = GENERATE(rerun::StoreKind::Recording, rerun::StoreKind::Blueprint);

    GIVEN("recording stream kind" << kind) {
        AND_GIVEN("a valid application id") {
            THEN("creating a new stream does not log an error") {
                rerun::RecordingStream stream = check_logged_error([&] {
                    return rerun::RecordingStream("rerun_example_test", std::string_view(), kind);
                });

                AND_THEN("it does not crash on destruction") {}

                AND_THEN("it reports the correct kind") {
                    CHECK(stream.kind() == kind);
                }
            }
        }

        // We changed to taking std::string_view instead of const char* and constructing such from nullptr crashes
        // at least on some C++ implementations.
        // If we'd want to support this in earnest we'd have to create out own string_view type.
        //
        // AND_GIVEN("a nullptr for the application id") {
        //     THEN("creating a new stream logs a null argument error") {
        //         check_logged_error(
        //             [&] { rerun::RecordingStream stream(nullptr, kind); },
        //             rerun::ErrorCode::UnexpectedNullArgument
        //         );
        //     }
        // }
        AND_GIVEN("invalid utf8 character sequence for the application id") {
            THEN("creating a new stream logs an invalid string argument error") {
                check_logged_error(
                    [&] { rerun::RecordingStream stream("\xc3\x28", std::string_view(), kind); },
                    rerun::ErrorCode::InvalidStringArgument
                );
            }
        }
    }
}

SCENARIO("RecordingStream can be set as global and thread local", TEST_TAG) {
    for (auto kind : std::array{rerun::StoreKind::Recording, rerun::StoreKind::Blueprint}) {
        GIVEN("a store kind" << kind) {
            WHEN("querying the current one") {
                auto& stream = rerun::RecordingStream::current(kind);

                THEN("it reports the correct kind") {
                    CHECK(stream.kind() == kind);
                }

                THEN("it is not enabled") {
                    CHECK(!stream.is_enabled());
                }
            }

            WHEN("creating a new stream") {
                rerun::RecordingStream stream("test", std::string_view(), kind);

                THEN("it can be set as global") {
                    stream.set_global();
                }
                THEN("it can be set as thread local") {
                    stream.set_thread_local();
                }

                // TODO(andreas): There's no way of telling right now if the set stream is
                // functional.
            }
        }
    }
}

SCENARIO("RecordingStream can be used for logging archetypes and components", TEST_TAG) {
    for (auto kind : std::array{rerun::StoreKind::Recording, rerun::StoreKind::Blueprint}) {
        GIVEN("a store kind" << kind) {
            WHEN("creating a new stream") {
                rerun::RecordingStream stream("test", std::string_view(), kind);

                GIVEN("component batches") {
                    auto batch0 = rerun::ComponentBatch::from_loggable<rerun::Position2D>(
                                      {{1.0, 2.0}, {4.0, 5.0}}
                    ).value_or_throw();
                    auto batch1 = rerun::ComponentBatch::from_loggable<rerun::Color>(
                                      {rerun::Color(0xFF0000FF)}
                    )
                                      .value_or_throw();
                    THEN("single component batch can be logged") {
                        stream.log("log_archetype-splat", batch0);
                        stream.log_static("log_archetype-splat", batch0);
                    }
                    THEN("component batches can be listed out") {
                        stream.log("log_archetype-splat", batch0, batch1);
                        stream.log_static("log_archetype-splat", batch0, batch1);
                    }
                    THEN("a collection of component batches can be logged") {
                        rerun::Collection<rerun::ComponentBatch> batches = {batch0, batch1};
                        stream.log("log_archetype-splat", batches);
                        stream.log_static("log_archetype-splat", batches);
                    }
                }
                GIVEN("component batches wrapped in `rerun::Result`") {
                    auto batch0 = rerun::ComponentBatch::from_loggable<rerun::Position2D>(
                        {{1.0, 2.0}, {4.0, 5.0}}
                    );
                    auto batch1 = rerun::ComponentBatch::from_loggable<rerun::Color>(
                        {rerun::Color(0xFF0000FF)}
                    );
                    THEN("single component batch can be logged") {
                        stream.log("log_archetype-splat", batch0);
                        stream.log_static("log_archetype-splat", batch0);
                    }
                    THEN("component batches can be listed out") {
                        stream.log("log_archetype-splat", batch0, batch1);
                        stream.log_static("log_archetype-splat", batch0, batch1);
                    }
                    THEN("collection of component batch results can be logged") {
                        rerun::Collection<rerun::Result<rerun::ComponentBatch>> batches = {
                            batch0,
                            batch1};
                        stream.log("log_archetype-splat", batches);
                        stream.log_static("log_archetype-splat", batches);
                    }
                }

                THEN("an archetype can be logged") {
                    stream.log(
                        "log_archetype-splat",
                        rerun::Points2D({rerun::Vec2D{1.0, 2.0}, rerun::Vec2D{4.0, 5.0}}
                        ).with_colors(rerun::Color(0xFF0000FF))
                    );
                    stream.log_static(
                        "log_archetype-splat",
                        rerun::Points2D({rerun::Vec2D{1.0, 2.0}, rerun::Vec2D{4.0, 5.0}}
                        ).with_colors(rerun::Color(0xFF0000FF))
                    );
                }
                THEN("several archetypes can be logged") {
                    stream.log(
                        "log_archetype-splat",
                        rerun::Points2D({rerun::Vec2D{1.0, 2.0}, rerun::Vec2D{4.0, 5.0}}
                        ).with_colors(rerun::Color(0xFF0000FF)),
                        rerun::Points2D({rerun::Vec2D{1.0, 2.0}, rerun::Vec2D{4.0, 5.0}}
                        ).with_colors(rerun::Color(0xFF0000FF))
                    );
                    stream.log_static(
                        "log_archetype-splat",
                        rerun::Points2D({rerun::Vec2D{1.0, 2.0}, rerun::Vec2D{4.0, 5.0}}
                        ).with_colors(rerun::Color(0xFF0000FF)),
                        rerun::Points2D({rerun::Vec2D{1.0, 2.0}, rerun::Vec2D{4.0, 5.0}}
                        ).with_colors(rerun::Color(0xFF0000FF))
                    );
                }

                // TODO(andreas): There's no way of telling right now if the set stream is
                // functional and where those messages went.
            }
        }
    }
}

SCENARIO("RecordingStream can log to file", TEST_TAG) {
    const char* test_path = "build/test_output";
    fs::create_directories(test_path);

    std::string test_rrd0 = std::string(test_path) + "test-file-0.rrd";
    std::string test_rrd1 = std::string(test_path) + "test-file-1.rrd";

    fs::remove(test_rrd0);
    fs::remove(test_rrd1);

    GIVEN("a new RecordingStream") {
        auto stream0 = std::make_unique<rerun::RecordingStream>("test");

        // We changed to taking std::string_view instead of const char* and constructing such from nullptr crashes
        // at least on some C++ implementations.
        // If we'd want to support this in earnest we'd have to create out own string_view type.
        //
        // AND_GIVEN("a nullptr for the save path") {
        //     THEN("then the save call returns a null argument error") {
        //         CHECK(stream0->save(nullptr).code == rerun::ErrorCode::UnexpectedNullArgument);
        //     }
        // }
        AND_GIVEN("valid save path " << test_rrd0) {
            AND_GIVEN("a directory already existing at this path") {
                fs::create_directory(test_rrd0);
                THEN("then the save call fails") {
                    CHECK(
                        stream0->save(test_rrd0).code ==
                        rerun::ErrorCode::RecordingStreamSaveFailure
                    );
                }
            }
            THEN("save call returns no error") {
                REQUIRE(stream0->save(test_rrd0).is_ok());

                THEN("a new file got immediately created") {
                    CHECK(fs::exists(test_rrd0));
                }

                WHEN("creating a second stream") {
                    auto stream1 = std::make_unique<rerun::RecordingStream>("test2");

                    WHEN("saving that one to a different file " << test_rrd1) {
                        REQUIRE(stream1->save(test_rrd1).is_ok());

                        WHEN("logging an archetype to the second stream") {
                            check_logged_error([&] {
                                stream1->log(
                                    "archetype",
                                    rerun::Points2D({
                                        rerun::Vec2D{1.0, 2.0},
                                        rerun::Vec2D{4.0, 5.0},
                                    })
                                );
                            });

                            THEN("after destruction, the second stream produced a bigger file") {
                                stream0.reset();
                                stream1.reset();
                                CHECK(fs::file_size(test_rrd0) < fs::file_size(test_rrd1));
                            }
                        }
                    }
                }
            }
        }
    }
}

void test_logging_to_connection(const char* address, const rerun::RecordingStream& stream) {
    // We changed to taking std::string_view instead of const char* and constructing such from nullptr crashes
    // at least on some C++ implementations.
    // If we'd want to support this in earnest we'd have to create out own string_view type.
    //
    // AND_GIVEN("a nullptr for the socket address") {
    //     THEN("then the connect call returns a null argument error") {
    //         CHECK(stream.connect(nullptr, 0.0f).code == rerun::ErrorCode::UnexpectedNullArgument);
    //     }
    // }
    AND_GIVEN("an invalid address for the socket address") {
        THEN("then the save call fails") {
            CHECK(
                stream.connect_tcp("definitely not valid!", 0.0f).code ==
                rerun::ErrorCode::InvalidSocketAddress
            );
        }
    }
    AND_GIVEN("a valid socket address " << address) {
        THEN("save call with zero timeout returns no error") {
            REQUIRE(stream.connect_tcp(address, 0.0f).is_ok());

            WHEN("logging an archetype and then flushing") {
                check_logged_error([&] {
                    stream.log(
                        "archetype",
                        rerun::Points2D({
                            rerun::Vec2D{1.0, 2.0},
                            rerun::Vec2D{4.0, 5.0},
                        })
                    );
                });

                stream.flush_blocking();

                THEN("does not crash") {
                    // No easy way to see if it got sent.
                }
            }
        }
    }
}

SCENARIO("RecordingStream can connect", TEST_TAG) {
    const char* address = "127.0.0.1:9876";
    GIVEN("a new RecordingStream") {
        rerun::RecordingStream stream("test-local");
        test_logging_to_connection(address, stream);
    }
    WHEN("setting a global RecordingStream and then discarding it") {
        {
            rerun::RecordingStream stream("test-global");
            stream.set_global();
        }
        GIVEN("the current recording stream") {
            test_logging_to_connection(address, rerun::RecordingStream::current());
        }
    }
}

SCENARIO("Recording stream handles invalid logging gracefully", TEST_TAG) {
    GIVEN("a new RecordingStream") {
        rerun::RecordingStream stream("test");

        AND_GIVEN("a valid path") {
            const char* path = "valid";

            AND_GIVEN("a cell with a null buffer") {
                rerun::ComponentBatch cell = {};
                cell.component_type = 0;

                THEN("try_log_data_row fails with UnexpectedNullArgument") {
                    CHECK(
                        stream.try_log_data_row(path, 1, &cell, true).code ==
                        rerun::ErrorCode::UnexpectedNullArgument
                    );
                }
            }
            AND_GIVEN("a cell with an invalid component type") {
                rerun::ComponentBatch cell = {};
                cell.component_type = RR_COMPONENT_TYPE_HANDLE_INVALID;
                cell.array = rerun::components::indicator_arrow_array();

                THEN("try_log_data_row fails with InvalidComponentTypeHandle") {
                    CHECK(
                        stream.try_log_data_row(path, 1, &cell, true).code ==
                        rerun::ErrorCode::InvalidComponentTypeHandle
                    );
                }
            }
        }
    }
}

SCENARIO("Recording stream handles serialization failure during logging gracefully", TEST_TAG) {
    GIVEN("a new RecordingStream and a valid entity path") {
        rerun::RecordingStream stream("test");
        const char* path = "valid";
        auto& expected_error = rerun::Loggable<BadComponent>::error;

        AND_GIVEN("an component batch result that failed serialization") {
            const auto component = BadComponent();

            expected_error.code =
                GENERATE(rerun::ErrorCode::Unknown, rerun::ErrorCode::ArrowStatusCode_TypeError);

            auto batch_result = rerun::ComponentBatch::from_loggable(component);

            THEN("calling log with that batch logs the serialization error") {
                check_logged_error([&] { stream.log(path, batch_result); }, expected_error.code);
            }
            THEN("calling log with a collection wrapping that batch logs the serialization error") {
                check_logged_error(
                    [&] { stream.log(path, rerun::Collection{batch_result}); },
                    expected_error.code
                );
            }
        }
        AND_GIVEN("an archetype that fails serialization") {
            auto archetype = BadArchetype();
            expected_error.code =
                GENERATE(rerun::ErrorCode::Unknown, rerun::ErrorCode::ArrowStatusCode_TypeError);

            THEN("calling log_archetype logs the serialization error") {
                check_logged_error([&] { stream.log(path, archetype); }, expected_error.code);
            }
            THEN("calling log_archetype forwards the serialization error") {
                CHECK(stream.try_log(path, archetype) == expected_error);
            }
        }
    }
}

SCENARIO("RecordingStream can set time without errors", TEST_TAG) {
    rerun::RecordingStream stream("test");

    SECTION("Setting time sequence does not log errors") {
        check_logged_error([&] { stream.set_time_sequence("my sequence", 1); });
    }
    SECTION("Setting time seconds does not log errors") {
        check_logged_error([&] { stream.set_time_seconds("my sequence", 1.0); });
    }
    SECTION("Setting time nanos does not log errors") {
        check_logged_error([&] { stream.set_time_nanos("my sequence", 1); });
    }
    SECTION("Setting time via chrono duration does not log errors") {
        using namespace std::chrono_literals;
        check_logged_error([&] { stream.set_time("duration", 1.0s); });
        check_logged_error([&] { stream.set_time("duration", 1000ms); });
    }
    SECTION("Setting time via chrono duration does not log errors") {
        check_logged_error([&] { stream.set_time("timepoint", std::chrono::system_clock::now()); });
    }
    SECTION("Resetting time does not log errors") {
        check_logged_error([&] { stream.reset_time(); });
    }
    SECTION("Can set time again after resetting the time") {
        check_logged_error([&] { stream.reset_time(); });
        check_logged_error([&] { stream.set_time_seconds("duration", 1.0f); });
    }

    SECTION("Disabling timeline does not log errors") {
        check_logged_error([&] { stream.disable_timeline("doesn't exist"); });
        check_logged_error([&] { stream.set_time_sequence("exists!", 123); });
        check_logged_error([&] { stream.disable_timeline("exists"); });
    }
}

SCENARIO("Global RecordingStream doesn't cause crashes", TEST_TAG) {
    // This caused a crash on Mac & Linux due to issues with cleanup order of global variables
    // in Rust vs C++.
    // See:
    // * https://github.com/rerun-io/rerun/issues/5697
    // * https://github.com/rerun-io/rerun/issues/5260
    static rerun::RecordingStream global_stream("global");
}
