//! Helper library for using win32 TH32Snapshot for traversing processes
use crate::windows_bindings::{
    CloseHandle, CreateToolhelp32Snapshot, Process32First, Process32Next, INVALID_HANDLE_VALUE,
    PROCESSENTRY32, TH32CS_SNAPPROCESS, TRUE,
};

/// Creates snapshot and keeps a handle to it
///
/// This object will keep windows resource handle, and release it at [`Drop`]
pub(crate) struct ProcSnapshot {
    /// Handle to process snapshot
    snapshot: *mut core::ffi::c_void,
    /// Buffer for enumeration, win32 will write here during iteration
    entry_buf: PROCESSENTRY32,
}

impl ProcSnapshot {
    /// Create a handle
    pub fn new() -> Option<Self> {
        // Safety: validity postcondition is checked, agruments are valid
        // (https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if snapshot == INVALID_HANDLE_VALUE {
            return None;
        }
        Some(Self {
            snapshot,
            entry_buf: PROCESSENTRY32 {
                dwSize: size_of::<PROCESSENTRY32>() as u32,
                cntUsage: 0,
                th32ProcessID: 0,
                th32DefaultHeapID: 0,
                th32ModuleID: 0,
                cntThreads: 0,
                th32ParentProcessID: 0,
                pcPriClassBase: 0,
                dwFlags: 0,
                szExeFile: core::array::from_fn(|_| 0),
            },
        })
    }
    /// Use to find a process via closure
    pub fn find(mut self, predicate: impl Fn(&PROCESSENTRY32) -> bool) -> Option<PROCESSENTRY32> {
        // Safety: snapshot is valid by construction, buffer is created from reference so is valid
        if (unsafe { Process32First(self.snapshot, &raw mut self.entry_buf) } != TRUE) {
            return None;
        }
        loop {
            if predicate(&self.entry_buf) {
                return Some(self.entry_buf);
            }
            // Safety: snapshot is valid by construction, buffer is created from reference so is valid
            if unsafe { Process32Next(self.snapshot, &raw mut self.entry_buf) } != TRUE {
                break;
            }
        }
        None
    }
}

impl Drop for ProcSnapshot {
    fn drop(&mut self) {
        // Safety: snapshot is valid by construction
        unsafe { CloseHandle(self.snapshot) };
    }
}
