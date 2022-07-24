load("@rules_rust//rust:defs.bzl","rust_library")
def rust_cxx_bridge(name, src, deps = []):
    native.alias(
        name = "%s/header" % name,
        actual = src + ".h",
    )

    native.alias(
        name = "%s/source" % name,
        actual = src + ".cc",
    )

    run_binary(
        name = "%s/generated" % name,
        srcs = [src],
        outs = [
            src + ".h",
            src + ".cc",
        ],
        args = [
            "$(location %s)" % src,
            "-o",
            "$(location %s.h)" % src,
            "-o",
            "$(location %s.cc)" % src,
        ],
        tool = "@cxx.rs//:codegen",
    )

    cc_library(
        name = name,
        srcs = [src + ".cc"],
        deps = deps + [":%s/include" % name],
    )

    cc_library(
        name = "%s/include" % name,
        hdrs = [src + ".h"],
    )

def cxxrs_library(name,rsrcs,csrcs,hdrs,rdeps = [],deps = []):
    rust_library(name = name,srcs = rsrcs,deps = rdeps + [":" + name + "/b/" + src for src in rsrcs] + [":" + name + "/impl"])
    for src in rsrcs:
        rust_cxx_bridge(name = name + "/b/" + src,src = src,deps = [":" + name + "/include"])
    cc_library(name = name + "/include",hdrs = hdrs,deps = deps)
    cc_library(name = name + "/impl",srcs = csrcs,deps = [":" + name + "/b/" + src + "/include" for src in rsrcs] + [":" + name + "/include"])