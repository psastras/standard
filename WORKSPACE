load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "cae37240fa3a529a330dab17271899376636e07e1e9d31b964109cd60cb60e35",
    strip_prefix = "rules_rust-78ac316a460c8e86739ebff975e23902b7a1800b",
    urls = [
        # Master branch as of 2020-12-05
        "https://github.com/bazelbuild/rules_rust/archive/78ac316a460c8e86739ebff975e23902b7a1800b.tar.gz",
    ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("//cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()