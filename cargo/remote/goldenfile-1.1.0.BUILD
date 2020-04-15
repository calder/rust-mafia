"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "goldenfile",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__difference__2_0_0//:difference",
        "@raze__tempfile__3_1_0//:tempfile",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.1.0",
    crate_features = [
    ],
)

# Unsupported target "readme_usage" with type "test" omitted
# Unsupported target "test" with type "test" omitted
