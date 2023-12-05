const LINK_LIBS: &[&str] = &[
    "Discovery-cpp",
    "EventstreamRpc-cpp",
    "GreengrassIpc-cpp",
    "IotDeviceCommon-cpp",
    "IotDeviceDefender-cpp",
    "IotIdentity-cpp",
    "IotJobs-cpp",
    "IotSecureTunneling-cpp",
    "IotShadow-cpp",
];

fn main() {
    let crt_root = std::env::var("DEP_AWS_CRT_CPP_ROOT").unwrap();
    let c_iot_root = std::env::var("DEP_AWS_C_IOT_ROOT").unwrap();

    println!("cargo:rerun-if-changed=aws-iot-device-sdk-cpp-v2");
    let out_dir = cmake::Config::new("aws-iot-device-sdk-cpp-v2")
        .define("CMAKE_PREFIX_PATH", [crt_root, c_iot_root].join(";"))
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_DEPS", "OFF")
        .define("BUILD_SAMPLES", "OFF")
        .define("BUILD_TESTING", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    for lib in LINK_LIBS {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}
