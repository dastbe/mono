package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "cli",
    srcs = ["src/main.rs"],
    deps = [
      "//source/rust/cargo:inquire",
      "//source/rust/cargo:reqwest",
      "//source/rust/cargo:serde",
      "//source/rust/cargo:serde_json",
    ],
)
