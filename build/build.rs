//! libnotcurses-sys build script

extern crate bindgen;
extern crate pkg_config;

mod nc_csource;

/// The notcurses version that we are aiming to support in the current release.
const NC_VERSION: &str = "3.0.0";

fn main() {
    let nc_src = nc_csource::NcCSource::new(NC_VERSION);

    // vendor nc?
    if cfg!(feature = "nc_vendor") {
        nc_src.vendor();
    } else if !cfg!(feature = "nc_compile") {
        // remove the vendored source only if we're not compiling it.
        nc_src.dont_vendor();
    }

    // compile nc?
    if cfg!(feature = "nc_compile") {
        nc_src.compile();
    }

    // MAYBE install nc?
    // #[cfg(feature = "nc_install")]
    // nc_src.install();

    let plib = pkg_config::Config::new()
        .atleast_version(NC_VERSION)
        .probe("notcurses")
        .expect("pkg-config couldn't find the notcurses library");

    // tell cargo to invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed=build/wrapper.h");

    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-D_XOPEN_SOURCE")
        .clang_arg(&nc_src.headers_include_string())
        // the input header we would like to generate builder for
        .header("build/wrapper.h")
        // generate comments, also from headers and not just doc comments (///)
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        // https://github.com/dankamongmen/notcurses/pull/2331#issuecomment-966211120
        .size_t_is_usize(true)
        // Remove warnings about improper_ctypes
        .blocklist_function("strtold")
        .blocklist_function("wcstold")
        .blocklist_function("socketpair")
        // only import functions from notcurses public API
        .blocklist_function("[^ns].*")
        .blocklist_function("n[^co].*")
        .blocklist_function("s[^i].*") // allow sig*
        // clean more unneeded types
        .blocklist_item("B[0-9]+")
        .blocklist_item("_BITS.*")
        .blocklist_item("_POSIX.*")
        .blocklist_item("__[A-Z].*")
        .blocklist_item("[ADHJ-MQ-Z].*")
        .blocklist_item("IN.*")
        .blocklist_item("IP[^R].*")
        .blocklist_item("ip.*")
        .blocklist_item("m.*")
        .blocklist_item("PF.*")
        .blocklist_item("MSG_.*")
        .blocklist_item("N[^C].*")
        .blocklist_type("_bindgen.*")
        // https://github.com/dankamongmen/notcurses/pull/1937
        // https://github.com/rust-lang/rust-bindgen/issues/1651
        .layout_tests(false)
        // Don't derive the Copy trait on types with destructors
        .no_copy("ncdirect")
        .no_copy("ncdplot")
        .no_copy("ncfdplane")
        .no_copy("ncmenu")
        .no_copy("ncmenu_item")
        .no_copy("ncmenu_section")
        .no_copy("ncmenu_options")
        .no_copy("ncmultiselector")
        .no_copy("ncmultiselector_options")
        .no_copy("ncmselector_item")
        .no_copy("ncplane")
        .no_copy("ncplane_options")
        .no_copy("ncprogbar")
        .no_copy("ncreader")
        .no_copy("ncreel")
        .no_copy("ncselector")
        .no_copy("ncselector_item")
        .no_copy("ncselector_options")
        .no_copy("ncuplot")
        .no_copy("ncdplot")
        .no_copy("ncsubproc")
        .no_copy("nctab")
        .no_copy("nctabbed")
        .no_copy("nctabbed_options")
        .no_copy("nctree")
        .no_copy("nctree_item")
        .no_copy("nctree_options")
        .no_copy("ncvisual")
        .no_copy("ncvisual_options")
        .no_copy("notcurses")
        .no_copy("notcurses_options")
        // try to derive more traits
        .derive_default(true)
        .derive_hash(true)
        .derive_partialord(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_eq(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    for d in plib.include_paths {
        builder = builder.clang_arg(format!("-I{}", d.to_string_lossy()));
    }

    // finish the builder and generate the builder.
    let bindings = builder.generate().expect("Unable to generate bindings");

    // write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(nc_src.root_path().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
