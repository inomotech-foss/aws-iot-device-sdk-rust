extern crate aws_c_common_sys;

#[cfg(windows)]
extern crate schannel;

#[cfg(target_os = "macos")]
extern crate security_framework;
