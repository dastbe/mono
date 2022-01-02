package(default_visibility = ["//visibility:public"])

load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@build_bazel_rules_nodejs//:index.bzl", "nodejs_binary")

ts_project(
    name = "lib",
    srcs = glob(["*.ts"]),
    deps = [
        "@npm//aws-cdk",
        "@npm//aws-cdk-lib",
    ],
)

nodejs_binary(
    name = "app",
    data = [
        ":static/hello.json",
        "//infra/images/example/echo:image.tar",
        ":lib",
    ],
    entry_point = ":app.ts",
    visibility = ["//visibility:public"],
)

# TODO(belldav): template this?
nodejs_binary(
    name = "cdk",
    args = [
        "--app $(location :app)",
    ],
    data = [
        "tsconfig.json",
        ":app",
        "@npm//aws-cdk",
    ],
    entry_point = "@npm//:node_modules/aws-cdk/bin/cdk",
    visibility = ["//visibility:public"],
)
