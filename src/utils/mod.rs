#[cfg(windows)]
pub mod hex;
#[cfg(any(target_os = "android", target_os = "linux", target_os = "macos", target_os = "freebsd"))]
mod unix;

#[cfg(any(target_os = "android", target_os = "linux", target_os = "macos", target_os = "freebsd"))]
pub use unix::*;
