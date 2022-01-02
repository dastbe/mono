load("@rules_rust//proto:proto.bzl", "rust_proto_library", "rust_grpc_library")
package(default_visibility = ["//source:__subpackages__"],)

proto_library(
    name = "proto",
    srcs = ["echo.proto"]
)

rust_proto_library(
    name = "rust_proto",
    deps = [":proto"],
)

rust_grpc_library(
    name = "rust_grpc",
    deps = [":proto"],
)
