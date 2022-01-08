load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

def jdk_repositories():
    maybe(
        http_archive,
        name = "corretto_jdk17_linux_x86_64",
        build_file = "@bazel_tools//tools/jdk:jdk.BUILD",
        # sha256 = "56a00b4b6d76cd1cc80f515a06fe1b36e3dfff2083d6fb97f5375ae5fe9dac2b",
        strip_prefix = "amazon-corretto-17.0.1.12.1-linux-x64",
        urls = [
            "https://corretto.aws/downloads/resources/17.0.1.12.1/amazon-corretto-17.0.1.12.1-linux-x64.tar.gz",
        ],
    )

    maybe(
        http_archive,
        name = "corretto_jdk17_macos_arm64",
        build_file = "@bazel_tools//tools/jdk:jdk.BUILD",
        sha256 = "56a00b4b6d76cd1cc80f515a06fe1b36e3dfff2083d6fb97f5375ae5fe9dac2b",
        strip_prefix = "amazon-corretto-17.jdk/Contents/Home",
        urls = [
            "https://corretto.aws/downloads/resources/17.0.1.12.1/amazon-corretto-17.0.1.12.1-macosx-aarch64.tar.gz",
        ],
    )

    maybe(
        http_archive,
        name = "corretto_jdk17_macos_x86_64",
        build_file = "@bazel_tools//tools/jdk:jdk.BUILD",
        sha256 = "56a00b4b6d76cd1cc80f515a06fe1b36e3dfff2083d6fb97f5375ae5fe9dac2b",
        strip_prefix = "amazon-corretto-17.jdk/Contents/Home",
        urls = [
            "https://corretto.aws/downloads/resources/17.0.1.12.1/amazon-corretto-17.0.1.12.1-macosx-x64.tar.gz",
        ],
    )
