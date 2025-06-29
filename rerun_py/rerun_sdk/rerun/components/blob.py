# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/blob.fbs".

# You can extend this class by creating a "BlobExt" class in "blob_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["Blob", "BlobBatch"]


class Blob(datatypes.Blob, ComponentMixin):
    """**Component**: A binary blob of data."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of BlobExt in blob_ext.py

    # Note: there are no fields here because Blob delegates to datatypes.Blob


class BlobBatch(datatypes.BlobBatch, ComponentBatchMixin):
    _COMPONENT_TYPE: str = "rerun.components.Blob"


# This is patched in late to avoid circular dependencies.
Blob._BATCH_TYPE = BlobBatch  # type: ignore[assignment]
