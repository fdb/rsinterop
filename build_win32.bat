pushd mesh
cargo build
popd

call "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x64

set CommonCompilerFlags=-MT -nologo -Gm- -GR- -EHa- -Od -Oi -WX -W4 -wd4100 -wd4018 -EHsc -FC -Z7
set CommonLinkerFlags=-opt:ref advapi32.lib ws2_32.lib userenv.lib shell32.lib msvcrt.lib user32.lib gdi32.lib winmm.lib opengl32.lib -subsystem:console -libpath:..\mesh\target\debug mesh.lib

IF NOT EXIST build mkdir build
pushd build

cl %CommonCompilerFlags% ..\desktop\main.cpp /link %CommonLinkerFlags%

popd
