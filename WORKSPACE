load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Fetch Skylib (required by Rust rules).
http_archive(
    name = "bazel_skylib",
    sha256 = "9a737999532daca978a158f94e77e9af6a6a169709c0cee274f0a4c3359519bd",
    strip_prefix = "bazel-skylib-1.0.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/1.0.0.tar.gz",
)

# Fetch Rust rules.
http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "926c733d6836f02a8949e66e3ca20085a28c13585aa581a0d3154d34fc9b8620",
    strip_prefix = "rules_rust-a1d8936161beddadddd17d2b1c335ceac77d1d53",
    urls = [
        # Master branch as of 2020-04-14
        "https://github.com/bazelbuild/rules_rust/archive/a1d8936161beddadddd17d2b1c335ceac77d1d53.tar.gz",
    ],
)

# Load Rust rules.
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

# Fetch Cargo dependencies.
load("//cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()
