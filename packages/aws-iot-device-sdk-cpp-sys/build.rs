fn main() {
    aws_c_builder::Config::new("aws-iot-device-sdk-cpp-v2")
        .aws_dependencies(["AWS_CRT_CPP", "AWS_C_IOT"])
        .link_libraries([
            "Discovery-cpp",
            "EventstreamRpc-cpp",
            "GreengrassIpc-cpp",
            "IotDeviceCommon-cpp",
            "IotDeviceDefender-cpp",
            "IotIdentity-cpp",
            "IotJobs-cpp",
            "IotSecureTunneling-cpp",
            "IotShadow-cpp",
        ])
        .run_bindgen(false)
        .build()
}
