load("@crates//:defs.bzl", "build_crates_from", "crates_from")
load("@rules_rust//rust:defs.bzl","rust_library")
load("//3rdparty/cxxrs:defs.bzl","cxxrs_library")
cxxrs_library(
    name = "fnvm",
    rsrcs = glob(["**/*.rs"]),
    rdeps = crates_from("//:Cargo.toml"),
    srcs = glob(["*.cpp"]),
    hdrs = glob(["*.hpp"])
)