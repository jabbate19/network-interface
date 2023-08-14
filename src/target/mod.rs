#[cfg(any(target_os = "android", target_os = "linux", target_os = "freebsd"))]
mod linux;

#[cfg(any(target_os = "android", target_os = "linux", target_os = "freebsd"))]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(target_os = "windows"))]
mod getifaddrs;

#[cfg(not(target_os = "windows"))]
pub use getifaddrs::*;
