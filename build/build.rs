// Docs:
// - https://rust-lang.github.io/rust-bindgen/tutorial-3.html
// - https://docs.rs/bindgen/*/bindgen/struct.Builder.html

extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let plib = pkg_config::Config::new()
        .atleast_version("2.4.8")
        .probe("notcurses")
        .unwrap();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=build/wrapper.h");

    // allow .blacklist_function instead of .blocklist_function for now,
    // until we update bindgen to >= 0.58.
    #[allow(deprecated)]
    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-D_XOPEN_SOURCE")
        // The input header we would like to generate builder for.
        .header("build/wrapper.h")
        // generate comments, also from headers and not just doc comments (///)
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        // Remove warnings about improper_ctypes
        .blacklist_function("strtold")
        .blacklist_function("wcstold")
        .blacklist_function("socketpair")
        // only import functions from notcurses public API
        .blacklist_function("[^ns].*")
        .blacklist_function("n[^co].*")
        .blacklist_function("s[^i].*") // allow sig*
        // clean more unneeded types
        .blacklist_item("B[0-9]+")
        .blacklist_item("_BITS.*")
        .blacklist_item("_POSIX.*")
        .blacklist_item("__[A-Z].*")
        .blacklist_item("[ADHJ-MQ-Z].*")
        .blacklist_item("IN.*")
        .blacklist_item("IP[^R].*")
        .blacklist_item("ip.*")
        .blacklist_item("m.*")
        .blacklist_item("PF.*")
        .blacklist_item("MSG_.*")
        .blacklist_item("N[^C].*")
        .blacklist_type("_bindgen.*")
        // https://github.com/dankamongmen/notcurses/pull/1937
        // https://github.com/rust-lang/rust-bindgen/issues/1651
        .layout_tests(false)
        // Don't derive the Copy trait on types with destructors.
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

    // Finish the builder and generate the builder.
    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
