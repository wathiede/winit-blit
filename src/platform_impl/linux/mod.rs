#![cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]

#[cfg(all(not(feature = "x11")))]
compile_error!("Please select a feature to build for unix: `x11`");

#[cfg(feature = "x11")]
pub mod x11;
pub use x11::*;
