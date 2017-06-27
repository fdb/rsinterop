@echo off
@call c:\emsdk\emsdk_env.bat
pushd webshim
cargo build --target=wasm32-unknown-emscripten --release
popd
IF NOT EXIST web\build mkdir web\build
del web\build\site.wasm
del web\build\site.js
del webshim\target\wasm32-unknown-emscripten\release\deps\*.asm.js
copy /y webshim\target\wasm32-unknown-emscripten\release\deps\*.wasm web\build\
copy /y webshim\target\wasm32-unknown-emscripten\release\deps\*.js web\build\
move /y web\build\*.wasm web\build\site.wasm
move /y web\build\*.js web\build\site.js
