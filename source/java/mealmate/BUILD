java_binary(
    name = "mealmate_bin",
    srcs = glob(["src/main/java/**/*.java"]),
    resources = glob(["src/main/resources/**"]),
    create_executable = False,
    deps = [
        "//source/java/common/lombok",
        "@maven//:com_amazon_alexa_ask_sdk",
        "@maven//:com_google_guava_guava",
        "@maven//:com_fasterxml_jackson_dataformat_jackson_dataformat_yaml"
    ],
)

java_library(
    name = "mealmate",
    srcs = glob(["src/main/java/**/*.java"]),
    resources = glob(["src/main/resources/**"]),
    deps = [
        "//source/java/common/lombok",
        "@maven//:com_amazon_alexa_ask_sdk",
        "@maven//:com_google_guava_guava",
        "@maven//:com_fasterxml_jackson_dataformat_jackson_dataformat_yaml"
    ],
)

java_test(
    name = "mealmate_test",
    srcs = glob(["src/test/java/**/*.java"]),
    resources = glob(["src/test/resources/**"]),
    use_testrunner = False,
    main_class = "org.junit.platform.console.ConsoleLauncher",
    args = ["--select-package", "dev.dbell.mealmate"],
    deps = [
        ":mealmate",
        "@maven//:org_junit_jupiter_junit_jupiter_api",
        "@maven//:org_junit_jupiter_junit_jupiter_engine",
        "@maven//:org_junit_jupiter_junit_jupiter_params",
        "@maven//:org_junit_platform_junit_platform_suite_api",
        "@maven//:org_apiguardian_apiguardian_api",
        "@maven//:org_opentest4j_opentest4j",
    ],
    runtime_deps = [
        "@maven//:org_junit_platform_junit_platform_commons",
        "@maven//:org_junit_platform_junit_platform_console",
        "@maven//:org_junit_platform_junit_platform_engine",
        "@maven//:org_junit_platform_junit_platform_launcher",
        "@maven//:org_junit_platform_junit_platform_suite_api",
    ],
)
