alias(
  name = "al2",
  actual = select({
    "//infra/conditions:x86_64": "@al2_x86_64//image:image",
    "//infra/conditions:arm64": "@al2_arm64//image:image",
  }),
)

alias(
  name = "base",
  actual = ":al2",
  visibility = ["//visibility:public"],
)
