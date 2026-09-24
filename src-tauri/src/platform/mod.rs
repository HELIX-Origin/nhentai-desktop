//! Platform-specific backend support.
//!
//! Each supported desktop OS has its own module implementing the identical
//! free-function surface. `platform` re-exports exactly one implementation at
//! compile time, so callers use `crate::platform::executable_name()` etc. and
//! never see the OS. Mobile targets are intentionally unsupported.
//!
//! Adding a platform = create `platform/xyz.rs` with the same functions and a
//! cfg block below. The compiler enforces signatures on that OS.

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

use std::path::Path;

/// The directory that already exists and is closest to `path` (walking up).
pub fn nearest_existing_dir(path: &Path) -> std::path::PathBuf {
    let mut probe = path.to_path_buf();
    loop {
        if probe.exists() {
            return probe;
        }
        match probe.parent() {
            Some(parent) => probe = parent.to_path_buf(),
            None => return probe,
        }
    }
}