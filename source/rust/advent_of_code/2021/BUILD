package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "problem_1",
    srcs = ["src/problem_1.rs"],
    data = ["src/problem_1.input"],
    deps = [
      "//source/rust/cargo:itertools",
    ],
)
