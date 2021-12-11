# ferrous-opengl
A 3D game engine written in Rust using the OpenGL wrapper glium

## Building and running the project from source
### Linux
```
git clone https://github.com/dillionnason/ferrous-opengl
cd ferrous-opengl
cargo build --release
./target/release/ferrous-opengl
```
### MacOS and Windows
Haven't tested building on Windows yet, and I lack the hardware to test on MacOS at the time

## Currently planned features
- Entity Component System
- Debugging Tools
- Rendering optimizations
- Potentially other APIs (Vulkan)

## Guides and references used in this project
- [Tutorial: Writing a tiny Entity Component System in Rust](https://ianjk.com/ecs-in-rust/)
- [kctess5's Simple_Renderer project](https://github.com/kctess5/Simple_Renderer)
- [jdah's minecraft-weekend](https://github.com/jdah/minecraft-weekend)
- [glium guides](https://github.com/glium/glium/tree/master/book)
