# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/archetypes/series_point.fbs".

# You can extend this class by creating a "SeriesPointExt" class in "series_point_ext.py".

from __future__ import annotations

from typing import Any

import numpy as np
from attrs import define, field

from .. import components, datatypes
from .._baseclasses import (
    Archetype,
    ComponentColumnList,
    DescribedComponentBatch,
)
from ..error_utils import catch_and_log_exceptions

__all__ = ["SeriesPoint"]


@define(str=False, repr=False, init=False)
class SeriesPoint(Archetype):
    """
    **Archetype**: Define the style properties for a point series in a chart.

    This archetype only provides styling information and should be logged as static
    when possible. The underlying data needs to be logged to the same entity-path using
    [`archetypes.Scalar`][rerun.archetypes.Scalar].

    Example
    -------
    ### Point series:
    ```python
    from math import cos, sin, tau

    import rerun as rr

    rr.init("rerun_example_series_point_style", spawn=True)

    # Set up plot styling:
    # They are logged as static as they don't change over time and apply to all timelines.
    # Log two point series under a shared root so that they show in the same plot by default.
    rr.log(
        "trig/sin",
        rr.SeriesPoint(
            color=[255, 0, 0],
            name="sin(0.01t)",
            marker="circle",
            marker_size=4,
        ),
        static=True,
    )
    rr.log(
        "trig/cos",
        rr.SeriesPoint(
            color=[0, 255, 0],
            name="cos(0.01t)",
            marker="cross",
            marker_size=2,
        ),
        static=True,
    )

    # Log the data on a timeline called "step".
    for t in range(0, int(tau * 2 * 10.0)):
        rr.set_time_sequence("step", t)

        rr.log("trig/sin", rr.Scalar(sin(float(t) / 10.0)))
        rr.log("trig/cos", rr.Scalar(cos(float(t) / 10.0)))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/1200w.png">
      <img src="https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(
        self: Any,
        *,
        color: datatypes.Rgba32Like | None = None,
        marker: components.MarkerShapeLike | None = None,
        name: datatypes.Utf8Like | None = None,
        marker_size: datatypes.Float32Like | None = None,
    ):
        """
        Create a new instance of the SeriesPoint archetype.

        Parameters
        ----------
        color:
            Color for the corresponding series.
        marker:
            What shape to use to represent the point
        name:
            Display name of the series.

            Used in the legend.
        marker_size:
            Size of the marker.

        """

        # You can define your own __init__ function as a member of SeriesPointExt in series_point_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(color=color, marker=marker, name=name, marker_size=marker_size)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            color=None,
            marker=None,
            name=None,
            marker_size=None,
        )

    @classmethod
    def _clear(cls) -> SeriesPoint:
        """Produce an empty SeriesPoint, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    @classmethod
    def update_fields(
        cls,
        *,
        clear: bool = False,
        color: datatypes.Rgba32Like | None = None,
        marker: components.MarkerShapeLike | None = None,
        name: datatypes.Utf8Like | None = None,
        marker_size: datatypes.Float32Like | None = None,
    ) -> SeriesPoint:
        """
        Update only some specific fields of a `SeriesPoint`.

        Parameters
        ----------
        clear:
            If true, all unspecified fields will be explicitly cleared.
        color:
            Color for the corresponding series.
        marker:
            What shape to use to represent the point
        name:
            Display name of the series.

            Used in the legend.
        marker_size:
            Size of the marker.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            kwargs = {
                "color": color,
                "marker": marker,
                "name": name,
                "marker_size": marker_size,
            }

            if clear:
                kwargs = {k: v if v is not None else [] for k, v in kwargs.items()}  # type: ignore[misc]

            inst.__attrs_init__(**kwargs)
            return inst

        inst.__attrs_clear__()
        return inst

    @classmethod
    def clear_fields(cls) -> SeriesPoint:
        """Clear all the fields of a `SeriesPoint`."""
        inst = cls.__new__(cls)
        inst.__attrs_init__(
            color=[],
            marker=[],
            name=[],
            marker_size=[],
        )
        return inst

    @classmethod
    def columns(
        cls,
        *,
        color: datatypes.Rgba32ArrayLike | None = None,
        marker: components.MarkerShapeArrayLike | None = None,
        name: datatypes.Utf8ArrayLike | None = None,
        marker_size: datatypes.Float32ArrayLike | None = None,
    ) -> ComponentColumnList:
        """
        Construct a new column-oriented component bundle.

        This makes it possible to use `rr.send_columns` to send columnar data directly into Rerun.

        The returned columns will be partitioned into unit-length sub-batches by default.
        Use `ComponentColumnList.partition` to repartition the data as needed.

        Parameters
        ----------
        color:
            Color for the corresponding series.
        marker:
            What shape to use to represent the point
        name:
            Display name of the series.

            Used in the legend.
        marker_size:
            Size of the marker.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            inst.__attrs_init__(
                color=color,
                marker=marker,
                name=name,
                marker_size=marker_size,
            )

        batches = [batch for batch in inst.as_component_batches() if isinstance(batch, DescribedComponentBatch)]
        if len(batches) == 0:
            return ComponentColumnList([])

        lengths = np.ones(len(batches[0]._batch.as_arrow_array()))
        columns = [batch.partition(lengths) for batch in batches]

        indicator_batch = DescribedComponentBatch(cls.indicator(), cls.indicator().component_descriptor())
        indicator_column = indicator_batch.partition(np.zeros(len(lengths)))

        return ComponentColumnList([indicator_column] + columns)

    color: components.ColorBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.ColorBatch._converter,  # type: ignore[misc]
    )
    # Color for the corresponding series.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    marker: components.MarkerShapeBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.MarkerShapeBatch._converter,  # type: ignore[misc]
    )
    # What shape to use to represent the point
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    name: components.NameBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.NameBatch._converter,  # type: ignore[misc]
    )
    # Display name of the series.
    #
    # Used in the legend.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    marker_size: components.MarkerSizeBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.MarkerSizeBatch._converter,  # type: ignore[misc]
    )
    # Size of the marker.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
