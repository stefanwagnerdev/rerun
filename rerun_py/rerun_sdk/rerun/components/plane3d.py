# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/plane3d.fbs".

# You can extend this class by creating a "Plane3DExt" class in "plane3d_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["Plane3D", "Plane3DBatch"]


class Plane3D(datatypes.Plane3D, ComponentMixin):
    """
    **Component**: An infinite 3D plane represented by a unit normal vector and a distance.

    Any point P on the plane fulfills the equation `dot(xyz, P) - d = 0`,
    where `xyz` is the plane's normal and `d` the distance of the plane from the origin.
    This representation is also known as the Hesse normal form.

    Note: although the normal will be passed through to the
    datastore as provided, when used in the Viewer, planes will always be normalized.
    I.e. the plane with xyz = (2, 0, 0), d = 1 is equivalent to xyz = (1, 0, 0), d = 0.5
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of Plane3DExt in plane3d_ext.py

    # Note: there are no fields here because Plane3D delegates to datatypes.Plane3D
    pass


class Plane3DBatch(datatypes.Plane3DBatch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.components.Plane3D")


# This is patched in late to avoid circular dependencies.
Plane3D._BATCH_TYPE = Plane3DBatch  # type: ignore[assignment]
