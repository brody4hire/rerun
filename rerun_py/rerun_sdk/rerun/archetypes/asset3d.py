# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/archetypes/asset3d.fbs".

# You can extend this class by creating a "Asset3DExt" class in "asset3d_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)
from .asset3d_ext import Asset3DExt

__all__ = ["Asset3D"]


@define(str=False, repr=False, init=False)
class Asset3D(Asset3DExt, Archetype):
    """
    **Archetype**: A prepacked 3D asset (`.gltf`, `.glb`, `.obj`, `.stl`, etc.).

    See also [`archetypes.Mesh3D`][rerun.archetypes.Mesh3D].

    If there are multiple [`archetypes.InstancePoses3D`][rerun.archetypes.InstancePoses3D] instances logged to the same entity as a mesh,
    an instance of the mesh will be drawn for each transform.

    Example
    -------
    ### Simple 3D asset:
    ```python
    import sys

    import rerun as rr

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <path_to_asset.[gltf|glb|obj|stl]>")
        sys.exit(1)

    rr.init("rerun_example_asset3d", spawn=True)

    rr.log("world", rr.ViewCoordinates.RIGHT_HAND_Z_UP, static=True)  # Set an up-axis
    rr.log("world/asset", rr.Asset3D(path=sys.argv[1]))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/1200w.png">
      <img src="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/full.png" width="640">
    </picture>
    </center>

    """

    # __init__ can be found in asset3d_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            blob=None,  # type: ignore[arg-type]
            media_type=None,  # type: ignore[arg-type]
            albedo_factor=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> Asset3D:
        """Produce an empty Asset3D, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    blob: components.BlobBatch = field(
        metadata={"component": "required"},
        converter=components.BlobBatch._required,  # type: ignore[misc]
    )
    # The asset's bytes.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    media_type: components.MediaTypeBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.MediaTypeBatch._optional,  # type: ignore[misc]
    )
    # The Media Type of the asset.
    #
    # Supported values:
    # * `model/gltf-binary`
    # * `model/gltf+json`
    # * `model/obj` (.mtl material files are not supported yet, references are silently ignored)
    # * `model/stl`
    #
    # If omitted, the viewer will try to guess from the data blob.
    # If it cannot guess, it won't be able to render the asset.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    albedo_factor: components.AlbedoFactorBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.AlbedoFactorBatch._optional,  # type: ignore[misc]
    )
    # A color multiplier applied to the whole asset.
    #
    # For mesh who already have albedo_factor in materials,
    # it will be overwritten by actual albedo_factor of Asset3D (if specified).
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
