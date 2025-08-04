# russimp-ng

[![Crates.io](https://img.shields.io/crates/v/russimp-ng.svg)](https://crates.io/crates/russimp-ng)
[![docs.rs](https://docs.rs/russimp-ng/badge.svg)](https://docs.rs/russimp-ng)
[![Build Status](https://github.com/Kek5chen/russimp-ng/workflows/CI/badge.svg?branch=master)](https://github.com/Kek5chen/russimp-ng/actions)
[![License](https://img.shields.io/crates/l/russimp-ng.svg)](LICENSE)

**Maintained Rust bindings for the [Open Asset Import Library (Assimp)](https://github.com/assimp/assimp).**

`russimp-ng` provides idiomatic and safe Rust bindings for the popular Assimp library, enabling you to load dozens of 3D model formats into a common, usable data structure.

This crate is a maintained fork of the original `russimp` library, aiming to provide ongoing updates, bug fixes, and community support.

## Features

* **Load 3D Models:** Supports a [wide variety of formats](https://github.com/assimp/assimp#supported-file-formats) (e.g., FBX, glTF, OBJ, BLEND, DAE, etc.).
* **Access Scene Data:** Retrieve meshes (vertices, faces, normals, UVs, colors), materials, textures, animations, bones, metadata, and scene hierarchy.
* **Post-Processing:** Apply various optional steps to your loaded data, such as triangulation, calculating tangent spaces, optimizing meshes, joining identical vertices, and more.
* **Flexible Loading:** Load models from file paths or directly from memory buffers.
* **Configurable Linking:** Choose how `russimp-ng` links to the underlying C/C++ Assimp library (system dynamic library, prebuilt static library, or build-from-source static library).

## Installation & Linking Assimp

First, add `russimp-ng` to your `Cargo.toml`:

```toml
[dependencies]
russimp-ng = "3.2.1"
````

Next, you need to decide how to link the Assimp C++ library. `russimp-ng` uses the `russimp-sys-ng` crate to manage this.

**Option 1: Dynamic Linking (Default)**

This is the default behavior. `russimp-ng` will look for a shared Assimp library installed on your system at runtime.

  * **macOS:** `brew update && brew install assimp`
  * **Linux (Debian/Ubuntu):** `sudo apt-get update && sudo apt-get install libassimp-dev`
  * **Linux (Fedora):** `sudo dnf install assimp-devel`
  * **Windows:** Requires manually installing the Assimp SDK or building Assimp and ensuring `assimp.dll` (or similar) is accessible in your system's `PATH`. Using the `prebuilt` feature (Option 2) is often simpler on Windows.

**Option 2: Prebuilt Static Binaries (Recommended for Simplicity/Windows)**

Use the `prebuilt` feature to download and statically link a precompiled version of Assimp during the build. This avoids needing Assimp installed system-wide and simplifies cross-platform builds.

```toml
[dependencies]
russimp-ng = { version = "3.2.1", features = ["prebuilt"] }
```

*Note: Prebuilt binaries are provided via the underlying `russimp-sys-ng` crate's releases.*

**Option 3: Build Assimp Statically from Source**

Use the `static-link` feature to download the Assimp source code, compile it, and link it statically into your binary.

```toml
[dependencies]
russimp-ng = { version = "3.2.1", features = ["static-link"] }
```

**Build Requirements for `static-link`:**

  * `cmake`
  * A C++ compiler toolchain (Clang is recommended)
  * Build system generator: `Ninja` (recommended for Linux/macOS) or Visual Studio 2019+ (Windows)

**Additional Feature Flags:**

  * `nozlib`: (Used with `static-link` or `prebuilt`) By default, Assimp bundles its own `zlib`. Enable this feature if you need to use a different `zlib` source/version already present in your project to avoid conflicts.

## Basic Usage

Load a scene from a file and apply some post-processing steps:

```rust
use russimp_ng::scene::{Scene, PostProcess};
use russimp_ng::error::RussimpError;
use std::path::Path;

fn load_model_info(file_path: &str) -> Result<(), RussimpError> {
    // Define the post-processing steps you want
    let post_process_flags = vec![
        PostProcess::CalculateTangentSpace,
        PostProcess::Triangulate,
        PostProcess::JoinIdenticalVertices,
        PostProcess::SortByPrimitiveType,
        PostProcess::GenerateNormals, // Example: Generate normals if missing
        PostProcess::GenerateUVCoords, // Example: Generate UVs if missing
        PostProcess::OptimizeMeshes,  // Example: Optimize mesh data
    ];

    // Load the scene from file
    let scene: Scene = Scene::from_file(file_path, post_process_flags)?;

    println!("Scene loaded successfully from: {}", file_path);
    println!("  Meshes: {}", scene.meshes.len());
    println!("  Materials: {}", scene.materials.len());
    println!("  Textures: {}", scene.textures.len());
    println!("  Animations: {}", scene.animations.len());
    println!("  Lights: {}", scene.lights.len());
    println!("  Cameras: {}", scene.cameras.len());

    // You can now access the scene data, e.g.:
    if let Some(first_mesh) = scene.meshes.first() {
        println!("  First mesh has {} vertices.", first_mesh.vertices.len());
    }

    // Scene can also be loaded from a byte buffer:
    // let buffer: &[u8] = ...;
    // let scene_from_buffer = Scene::from_buffer(buffer, post_process_flags, "fbx")?; // Provide format hint

    Ok(())
}

fn main() {
    let model_path = "assets/model.fbx";

    match load_model_info(model_path) {
        Ok(_) => println!("Finished processing model."),
        Err(e) => eprintln!("Error loading model: {}", e),
    }
}
```

## Examples

Check the `examples/` directory in this repository for more usage examples.

## Documentation

Detailed API documentation can be found on [docs.rs](https://docs.rs/russimp-ng/3.2.1).

## Contributing

Contributions are highly welcome\! Whether it's reporting bugs, suggesting features, improving documentation, or submitting pull requests, your help is appreciated.

  * Please check the [Issue Tracker](https://github.com/Kek5chen/russimp-ng/issues) for existing tasks or to create a new one.
  * Ensure your code is formatted with `cargo fmt`.
  * Please include tests for new features or bug fixes.

## License

This project is licensed the terms mentioned in [LICENSE](https://github.com/Kek5chen/LICENSE). It builds upon the original `russimp` project and the Assimp library.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of notable changes.

## Acknowledgements

  * This library relies heavily on the fantastic [Assimp](https://github.com/assimp/assimp) project.
  * Thanks to the original authors and contributors of `russimp`.

<!-- end list -->

