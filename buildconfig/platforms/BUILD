package(
    default_visibility = ["//build/testutil:__pkg__"],
)

_MACOS_CPUS = [
   "x86_64",
   "aarch64",
]

_LINUX_CPUS = [
   "x86_64",
   "aarch64",
]

[platform(
   name = "macos_{}".format(cpu),
   constraint_values = [
      "@platforms//os:macos",
      "@platforms//cpu:{}".format(cpu),
   ],
) for cpu in _MACOS_CPUS]

[platform(
   name = "linux_{}".format(cpu),
   constraint_values = [
      "@platforms//os:linux",
      "@platforms//cpu:{}".format(cpu),
   ],
) for cpu in _LINUX_CPUS]

