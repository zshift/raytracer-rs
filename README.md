# raytracer-rs
Raytracer implementation in rust.

This was heavily inspired by https://github.com/bheisler/raytracer 
(I followed his blog posts on creating the raytracer, starting with https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/).
I've since added parallelism with [rayon](https://crates.io/crates/rayon), and I'm currently working on supporting multiple color types.

# Running

Debug builds can be run with `cargo run --bin render <scene.json> <image.png>`. 

Release mode is built with `cargo build --release`, and the executable will be `target/release/render`. The library will  be `target/release/libraytracer.rlib`.

# Sample Output

![raytraced-4k-render](./samples/4k.png)