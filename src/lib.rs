//! This is a zero-dependency (implicitly depends on win32 via `windows-bindgen`) library to achieve multi-subsystem single executable programs on Windows.
//!
//! The method used in this library will flash a console for a brief period,
//! but otherwise this will allow you to create an application with both GUI and CLI codepaths.
//!
//! Motivating example:
//! ```ignore
//! // Optionally specify at the top of "main.rs" ("console" is default)
//! #![windows_subsystem = "console"]
//!
//! let is_gui = true; // parse arguments and determine console/windowed mode
//! if (is_gui) {
//!     #[cfg(windows)]
//!     hide_console_ng::hide_console();
//! }
//! // continue with your application
//! ```
//!
//! This example allows to run GUI application and hide a console depending on if application was started from console or from Windows GUI Shell.
//!
//! This is done by detecting if our app parent process is "explorer.exe".
//!
//! The behaviour resembles how many Linux desktops will run an app from desktop shortcuts without attaching a console,
//! but if you run an executable from terminal it will keep connected to parent shell's stdout/stdin.
//!
//! This also is beneficial since text selection in console can pause the execution of main thread on Windows, which is undesirable for GUI application.
//!
//! This library fails to compile on non-windows targets, to encourage use of conditional dependencies in `Cargo.toml`:
//!
//! ```toml
//! [target.'cfg(windows)'.dependencies]
//! hide_console_ng = "0"
//! ```
//!
//! Additionally [`show_unconditionally`] and [`hide_unconditionally`] is provided for completeness.
//!

#[cfg(not(windows))]
compile_error!("Don't include this dependency in targets other than windows");
mod proc_snapshot;
mod show_window;
mod windows_bindings;

/// Hide console if an app is started via `explorer.exe`
///
/// If used in a `windows_subsystem = "windows"` environment, the console is not attached, so this will do nothing
pub fn hide_console() {
    show_window::show_window(show_window::Action::HideFromExplorer);
}

/// Hide console even if parent process is not Explorer
///
/// This may be finnicky and may not work when used from custom consoles like "Windows Terminal"
/// (see <https://github.com/microsoft/terminal/issues/15311>).
///
/// If used in a `windows_subsystem = "windows"` environment, the console is not attached, so this will do nothing
pub fn hide_unconditionally() {
    show_window::show_window(show_window::Action::Hide);
}

/// Show console
///
/// If used in a `windows_subsystem = "windows"` environment, the console is not attached, so this will do nothing
pub fn show_unconditionally() {
    show_window::show_window(show_window::Action::Show);
}
