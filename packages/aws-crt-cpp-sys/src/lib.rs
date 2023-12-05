#[cfg(windows)]
extern crate schannel;

#[cfg(target_os = "macos")]
extern crate security_framework;
