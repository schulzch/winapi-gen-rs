# winapi-gen

This is a highly experimental crate to generate bindings from `windows.h` using `rustgen`. Use the `winapi` crate instead!

# Getting started

- run `"%VS140COMNTOOLS%..\..\VC\vcvarsall.bat" x86_amd64`
- run `cargo build`

# Issues

- `#define`: cast-style constants, e.g., `#define CONSTANT ((int)123)`
- `#define`: functions, e.g., `LOWORD(value)`
- better approach to detection of `extern "C"` vs. `extern "system"`
- probably more things I missed...
