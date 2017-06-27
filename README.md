# Rust Interop

Experiments on using Rust code as a library on the desktop and on the web.

This example is a "mesh" library in Rust that generates 3D meshes that we want to render on the desktop and the web.
We **don't** want to do rendering on the Rust side: Rust only returns data structures. It's up to the library user to render the mesh.
This model is similar to how [Dear ImGui](https://github.com/ocornut/imgui) works.

## Project structure

```
desktop - The C++ desktop application.
mesh    - The core Rust library we're developing. It generates meshes for us to render.
web     - The web project, containing the index.html. We will create a build folder and populate it with the WebAssembly files.
webshim - Because we can't directly expose a Rust library on the web, the shim wraps the library in a binary module.
```

## Prerequisites

* Install [Rustup](https://www.rustup.rs/).
* Open a shell, then install the Rust WebAssembly target:
```
rustup target add wasm32-unknown-emscripten
```
* Download the [Emscripten Portable SDK](http://kripken.github.io/emscripten-site/docs/getting_started/downloads.html). Unzip it in `~/emsdk` on macOS and Linux, or `C:\emsdk` on Windows.
* On Windows, add `c:\emsdk` to the `PATH`.
* Open a shell and install the latest Emscripten version:
```
emsdk update
emsdk install latest
emsdk activate latest
```
* You should be able to call the compiler:
```
emcc -v
```

* If you want to build a desktop application on Windows, install [Visual Studio 2017](https://www.visualstudio.com/vs/) (older versions might work as well, but you'll have to change the paths in `build_win32.bat`).
* Finally, clone this project: `git clone https://github.com/fdb/rsinterop.git && cd rsinterop`

## Building for the web

* Build: `./build_web` (on macOS and Linux) or `build_web.bat` (on Windows)

## Building for Windows

* Run `build_win32.bat` (on Windows)

## Useful Resources
* [Compiling Rust to WebAssembly Guide](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde)
* [The Path to Rust on the Web](http://asquera.de/blog/2017-04-10/the-path-to-rust-on-the-web/)
