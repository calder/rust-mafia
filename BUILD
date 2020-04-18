package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library", "rust_test")

rust_binary(
    name = "mafia_bin",
    srcs = ["//src:bin.rs"],
    deps = [
        ":mafia",
    ],
)

rust_library(
    name = "mafia",
    srcs = [
        "//src:lib.rs",
    ],
)
