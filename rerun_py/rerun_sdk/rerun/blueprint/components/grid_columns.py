# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/grid_columns.fbs".

# You can extend this class by creating a "GridColumnsExt" class in "grid_columns_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["GridColumns", "GridColumnsBatch"]


class GridColumns(datatypes.UInt32, ComponentMixin):
    """**Component**: How many columns a grid container should have."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of GridColumnsExt in grid_columns_ext.py

    # Note: there are no fields here because GridColumns delegates to datatypes.UInt32
    pass


class GridColumnsBatch(datatypes.UInt32Batch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.blueprint.components.GridColumns")


# This is patched in late to avoid circular dependencies.
GridColumns._BATCH_TYPE = GridColumnsBatch  # type: ignore[assignment]
