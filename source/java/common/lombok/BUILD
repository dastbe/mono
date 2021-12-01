java_library(
    name = "lombok",
    exports = [
        "@maven//:org_projectlombok_lombok",
    ],
    exported_plugins = [
        ":lombok_plugin"
    ],
    visibility = ["//visibility:public"],
)

java_plugin(
    name = "lombok_plugin",
    processor_class = "lombok.launch.AnnotationProcessorHider$AnnotationProcessor",
    deps = [
        "@maven//:org_projectlombok_lombok",
    ],
)
