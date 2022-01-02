load("@build_bazel_rules_nodejs//:index.bzl", "nodejs_binary")

nodejs_binary(
    name = "cdk",
    args = [
        "--app $(location //infra/resources:app)",
    ],
    data = [
        "tsconfig.json",
        "//infra/resources:lib",
        "//infra/resources:app",
        "@npm//:node_modules",
    ],
    entry_point = "@npm//:node_modules/aws-cdk/bin/cdk",
    # https://github.com/bazelbuild/rules_nodejs/pull/2344
    templated_args = ["--bazel_patch_module_resolver"],
    visibility = ["//visibility:public"],
)
