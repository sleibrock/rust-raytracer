# Raytracer in Rust

A rudimentary raytracer in [Rust](https://rust-lang.org/).

## Goals

* Working raytrace implementation with basic shapes
* Different shading materials with Lambert, Metal and Glass
* Scene, Camera and Render API
* Shapes including Planes, Spheres, Triangles, Disks and Rectangles
* Antialiasing, gamma correction and working aspect ratio
* Depth of field with aperture and focus distance settings


## TODOs

Updated as of 12/13/2018

* Add an Axis-Aligned Bounding Box system (AABB tree collision detection)
* Add in mesh-based intersections to be able to read .OBJ format
* Voxel-raytracing
* Multi-threading with multiple-producer single-consumer
* Adding in .X3D format support to import Blender export data
* Lights, including spot-lighting and ambient lighting
* Camera animation / rendering multiple frames

## Layout of Code

This raytracer library is broken up into several smaller modules to modularize it as much as possible.

* `raytracer::math` contains all raw math functions and libraries for computing 3-dimension vectors
* `raytracer::geometry` contains all shapes and AABB-related tools
* `raytracer::shading` contains lights and materials
* `raytracer::rendering` contains tools for creating and rendering scenes


## Install

Clone and run the code with
```bash
git clone https://github.com/sleibrock/rust-raytracer
cd rust-raytracer
cargo run
```

Cargo must be installed as well as a Rust toolchain. For installing Rust, please visit [Rustup](https://rustup.rs)
