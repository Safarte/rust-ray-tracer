# Rust Ray Tracer

A simple 3D rendering engine written in Rust, inspired by the series of books: "[Ray Tracing In One Weekend](https://raytracing.github.io)" by Peter Shirley and by the excellent guides on [scratchapixel.com](https://www.scratchapixel.com/). This engine is able to render 3D scenes using Sphere, Cuboid, Triangle and Mesh primitives combined with Lambertian, Metallic, Dielectric or Emissive materials.

## Usage

You first need to build the project using `cargo build --release`, you will then be able to run the executable located in the `target/release/` folder.

The arguments needed to run the program are defined as follows:

```shell
USAGE:
    rust-ray-tracer [OPTIONS] <HEIGHT> <SAMPLES>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --aspect_ratio <FLOAT>     Sets the camera aspect ratio
    -g, --gltf <FILE>              Sets the input glTF scene file
    -o, --output <FILE>            Sets the output image file name
    -t, --threads <NUM_THREADS>    Sets the desired number of threads

ARGS:
    <HEIGHT>     Sets the image height
    <SAMPLES>    Sets the number of samples per pixel
```

By default the program will use all CPU cores to perform the rendering task.
