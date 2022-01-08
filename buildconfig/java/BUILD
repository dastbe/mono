load("@bazel_tools//tools/jdk:default_java_toolchain.bzl", "default_java_toolchain")

# Copied from JDK9_JVM_OPTS in default_java_toolchain.bzl
JDK17_JVM_OPTS = [
    # Allow JavaBuilder to access internal javac APIs.
    "--add-exports=jdk.compiler/com.sun.tools.javac.api=ALL-UNNAMED",
    "--add-exports=jdk.compiler/com.sun.tools.javac.main=ALL-UNNAMED",
    "--add-exports=jdk.compiler/com.sun.tools.javac.model=ALL-UNNAMED",
    "--add-exports=jdk.compiler/com.sun.tools.javac.parser=ALL-UNNAMED",
    "--add-exports=jdk.compiler/com.sun.tools.javac.processing=ALL-UNNAMED",
    "--add-exports=jdk.compiler/com.sun.tools.javac.tree=ALL-UNNAMED",
    "--add-exports=jdk.compiler/com.sun.tools.javac.util=ALL-UNNAMED",
    "--add-opens=jdk.compiler/com.sun.tools.javac.code=ALL-UNNAMED",
    "--add-opens=jdk.compiler/com.sun.tools.javac.comp=ALL-UNNAMED",
    "--add-opens=jdk.compiler/com.sun.tools.javac.file=ALL-UNNAMED",

    # quiet warnings from com.google.protobuf.UnsafeUtil,
    # see: https://github.com/google/protobuf/issues/3781
    # and: https://github.com/bazelbuild/bazel/issues/5599
    "--add-opens=java.base/java.nio=ALL-UNNAMED",
    "--add-opens=java.base/java.lang=ALL-UNNAMED",
]

alias(
    name = "jdk",
    actual = select(
        {
            "//infra/conditions:macos_arm64": "@corretto_jdk17_macos_arm64//:jdk",
            "//infra/conditions:macos_x86_64": "@corretto_jdk17_macos_x86_64//:jdk",
            "//infra/conditions:linux_x86_64": "@corretto_jdk17_linux_x86_64//:jdk",
        },
        no_match_error = "Could not find a JDK",
    ),
)

default_java_toolchain(
    name = "java11_toolchain",
    bootclasspath = ["@bazel_tools//tools/jdk:platformclasspath"],
    jvm_opts = JDK17_JVM_OPTS,
    source_version = "11",
    target_version = "11",
    visibility = ["//visibility:public"],
)
