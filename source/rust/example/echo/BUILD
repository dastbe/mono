load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//proto:toolchain.bzl", "GRPC_COMPILE_DEPS")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "echo_server",
    srcs = ["src/server.rs"],
    deps = [
      "//idl/example/echo:rust_grpc",
    ] + GRPC_COMPILE_DEPS,
)

rust_binary(
    name = "echo_client",
    srcs = ["src/client.rs"],
    deps = [
      "//idl/example/echo:rust_grpc",
    ] + GRPC_COMPILE_DEPS,
)
