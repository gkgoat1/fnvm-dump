load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "531bdd470728b61ce41cf7604dc4f9a115983e455d46ac1d0c1632f613ab9fc3",
    strip_prefix = "rules_rust-d8238877c0e552639d3e057aadd6bfcf37592408",
    urls = [
        # `main` branch as of 2021-08-23
        "https://github.com/bazelbuild/rules_rust/archive/d8238877c0e552639d3e057aadd6bfcf37592408.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("//3rdparty/rules_rust:crate_universe_defaults.bzl", "DEFAULT_URL_TEMPLATE", "DEFAULT_SHA256_CHECKSUMS")

load("@rules_rust//crate_universe:defs.bzl", "crate", "crate_universe")

crate_universe(
    name = "crates",
    cargo_toml_files = [
        "//:Cargo.toml",
    ],
    resolver_download_url_template = DEFAULT_URL_TEMPLATE,
    resolver_sha256s = DEFAULT_SHA256_CHECKSUMS,
    # [package.metadata.raze.xxx] lines in Cargo.toml files are ignored;
    # the overrides need to be declared in the repo rule instead.
    # to use a lockfile, uncomment the following line,
    # create an empty file in the location, and then build
    # with REPIN=1 bazel build ...
    #lockfile = "//:crate_universe.lock",
)

load("@crates//:defs.bzl", "pinned_rust_install")

pinned_rust_install()
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
git_repository(
    name = "cxx.rs",
    remote = 'https://github.com/dtolnay/cxx.git',
    branch = "master"
)