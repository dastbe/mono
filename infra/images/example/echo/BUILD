load("@io_bazel_rules_docker//container:container.bzl", "container_image",)
package(default_visibility = ["//infra:__subpackages__"])

container_image(
    name = "image",
    base = "//infra/images/base",
    directory = "/opt/dastbe",
    workdir = "/opt/dastbe",
    files = ["//source/rust/example/echo:echo_server"],
    cmd = "./echo_server",
)
