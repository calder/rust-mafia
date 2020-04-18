package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

filegroup(
    name = "all",
    srcs = glob([
        "**/*.rs",
        "*.rs",
    ]),
)

rust_library(
    name = "game",
    srcs = ["game.rs"],
)

rust_test(
    name = "game_test",
    crate = ":game",
)
