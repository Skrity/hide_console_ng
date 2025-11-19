//! TODO: use cargo script when stable
const ARGS: &[&str] = &[
    "--out",
    "../src/windows_bindings.rs",
    "--config",
    "flatten",
    "std",
    "minimal",
    "--filter",
    "Windows.Win32.Foundation.TRUE",
    "Windows.Win32.Foundation.INVALID_HANDLE_VALUE",
    "Windows.Win32.Foundation.CloseHandle",
    "Windows.Win32.System.Console.GetConsoleWindow",
    "Windows.Win32.UI.WindowsAndMessaging.SW_HIDE",
    "Windows.Win32.UI.WindowsAndMessaging.SW_SHOW",
    "Windows.Win32.UI.WindowsAndMessaging.ShowWindow",
    "Windows.Win32.System.Diagnostics.ToolHelp.PROCESSENTRY32",
    "Windows.Win32.System.Diagnostics.ToolHelp.CreateToolhelp32Snapshot",
    "Windows.Win32.System.Diagnostics.ToolHelp.Process32First",
    "Windows.Win32.System.Diagnostics.ToolHelp.Process32Next",
    "Windows.Win32.System.Diagnostics.ToolHelp.TH32CS_SNAPPROCESS",
];

fn main() {
    windows_bindgen::bindgen(ARGS).unwrap();
}
