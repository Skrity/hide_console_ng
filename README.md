# Hide Console Next Generation

This is a zero-dependency (implicitly depends on win32 via `windows-bindgen`) library to achieve multi-subsystem single executable programs on Windows.

If used in a `#![windows_subsystem = "windows"]` environment, the console is not attached, so this won't do anything

## Console hiding methods comparison

|                                         | windows_subsystem = "windows" | windows_subsystem = "windows & AllocConsole | windows_subsystem = "windows" & AttachConsole | windows_subsystem = "console" & ShowWindow(SW_HIDE) |
|-----------------------------------------|-------------------------------|---------------------------------------------|-----------------------------------------------|-----------------------------------------------------|
| runtime selection                       | ❌                            | ✅                                          | ✅                                            | ✅                                                  |
| console doesn't flash at startup        | ✅                            | ✅                                          | ✅                                            | ❌                                                  |
| keeps parent's console and stdin/stdout | ❌(detached)                  | ❌*                                         | ✅**                                          | ✅                                                  |

\* new console is created outside of terminal application
** will attach stdin/stdout, but the console is not owned by our process

The method used in this library will **flash a console for a brief period**,
but otherwise this will allow you to create an application with both GUI and CLI codepaths.

## Features

- Zero dependencies
- Detects being run from explorer.exe, otherwise does nothing

## Documentation

```sh
cargo doc --target x86_64-pc-windows-gnu
```

## Installation

Add the library:

```toml
[target.'cfg(windows)'.dependencies]
hide_console_ng = "0"
```

## Usage

```rust
// Optionally specify at the top of "main.rs" ("console" is default)
#![windows_subsystem = "console"]

let is_gui = true; // parse arguments and determine console/windowed mode
if (is_gui) {
    #[cfg(windows)]
    hide_console_ng::hide_console();
}
// continue with your application
```
