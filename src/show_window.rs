use crate::proc_snapshot::ProcSnapshot;

use crate::windows_bindings::{GetConsoleWindow, ShowWindow, SW_HIDE, SW_SHOW};

/// Test if array contains "explorer.exe" cstring
#[inline]
fn is_explorer_exe(ascii: &[i8; 260]) -> bool {
    let explorer = b"explorer.exe\0";
    ascii[..explorer.len()]
        .iter()
        .zip(explorer.iter())
        .all(|(lhs, rhs)| lhs.cast_unsigned() == *rhs)
}

/// Perform an action on a console attached to our process.
///
/// This will only do something in `windows_subsystem = "console"`
///
/// If used in a `windows_subsystem = "windows"` environment, the console is not attached, so this will do nothing
pub(crate) fn show_window(action: Action) {
    // SAFETY: no requirements
    let window = unsafe { GetConsoleWindow() };

    if window.is_null() {
        // nothing to hide/show anyway
        return;
    }
    let action = match action {
        Action::Show => SW_SHOW,
        Action::Hide => SW_HIDE,
        Action::HideFromExplorer => {
            let my_pid = std::process::id();
            let Some(parent_pid) = ProcSnapshot::new()
                .and_then(|p| p.find(|e| e.th32ProcessID == my_pid))
                .map(|x| x.th32ParentProcessID)
            else {
                // didn't find ourselves - quite unlikely
                return;
            };
            let Some(parent) =
                ProcSnapshot::new().and_then(|p| p.find(|e| e.th32ProcessID == parent_pid))
            else {
                // didn't find parent: explorer.exe should exist, so bounce
                return;
            };
            if !is_explorer_exe(&parent.szExeFile) {
                // we were not ran from explorer, so hiding is not needed
                return;
            }
            SW_HIDE
        }
    };

    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    // SAFETY: `window` is a valid handle
    unsafe { ShowWindow(window, action) };
}

/// An action to perform when calling [`show_window`]
pub(crate) enum Action {
    /// This will show console, if there is a console to show
    Show,
    /// This will hide console, if there is a console to hide
    Hide,
    /// This will hide console if the program is being ran from desktop application
    ///
    /// This achieves pseudo-windows subsystem by checking if parent process is `explorer.exe`
    HideFromExplorer,
}
