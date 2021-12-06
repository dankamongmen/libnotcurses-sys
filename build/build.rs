// Docs:
// - https://rust-lang.github.io/rust-bindgen/tutorial-3.html
// - https://docs.rs/bindgen/*/bindgen/struct.Builder.html

extern crate bindgen;
extern crate pkg_config;
use std::{env::var, path::PathBuf};

#[cfg(feature = "compile_nc")]
extern crate cc;
#[cfg(feature = "compile_nc")]
use std::{
    env::set_var,
    fs::create_dir_all,
    path::Path,
    process::Command,
};

const VERSION: &str = "3.0.0";

fn main() {
    let build_out_path = PathBuf::from(var("OUT_DIR").unwrap());
    // println!("cargo:warning=OUT_DIR: {:?}", build_out_path);

    // compile notcurses
    #[cfg(feature = "compile_nc")]
    let nc_src_path = { compile_nc(&build_out_path) };
    #[cfg(feature = "compile_nc")]
    let nc_include_path = nc_src_path.join("include");
    // tell bindgen where are the headers
    #[cfg(feature = "compile_nc")]
    let nc_headers_path = format!["-I{}", nc_include_path.to_string_lossy()];
    #[cfg(not(feature = "compile_nc"))]
    let nc_headers_path = "".to_owned();

    let plib = pkg_config::Config::new()
        .atleast_version(VERSION)
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
        .clang_arg(nc_headers_path)
        // The input header we would like to generate builder for.
        .header("build/wrapper.h")
        // generate comments, also from headers and not just doc comments (///)
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        // https://github.com/dankamongmen/notcurses/pull/2331#issuecomment-966211120
        .size_t_is_usize(true)
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
    bindings
        .write_to_file(build_out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Downloads, compiles, and installs the `notcurses` C library.
///
/// This is mainly created for <docs.rs>, running on Ubuntu.
#[cfg(feature = "compile_nc")]
fn compile_nc(build_out_path: &Path) -> PathBuf {
    // clone notcurses repository with git

    // the path to the source directory
    let src_path = build_out_path.join(&format!["notcurses-{}", VERSION]);
    // println!("cargo:warning=src_path: {:?}", src_path);

    run(Command::new("git")
        .arg("clone")
        .arg("https://github.com/dankamongmen/notcurses")
        .arg(&src_path)
        .current_dir(&build_out_path));

    // switch to the desired tag
    run(Command::new("git")
        .arg("checkout")
        .arg(&format!["v{}", VERSION])
        .current_dir(&src_path));

    // install notcurses dependencies
    //
    // NOTE: for now it assumes "apt" is available (debian/ubuntu based distro).
    // This works well for docs.rs but it may ask for password in other systems.
    run(
        Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("libunistring-dev")
            .arg("libdeflate-dev")
            .arg("doctest-dev"), // .arg("pandoc") // not needed
    );

    // prepare the building directory
    let src_build_path = &src_path.join("build");
    create_dir_all(&src_build_path).expect("couldn't create 'build/' directory");

    set_var("PKG_CONFIG_PATH", &src_build_path);
    set_var("CPATH", &src_build_path.join("include/notcurses"));

    // compile notcurses
    run(Command::new("cmake")
        // .arg("-DCMAKE_INSTALL_PREFIX=/usr/local/") // (disabled install)
        .arg("-DUSE_DOCTEST=off")
        .arg("-DUSE_PANDOC=off")
        .arg("..")
        .current_dir(&src_build_path));

    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&src_build_path));

    // (disabled) install notcurses
    //
    // run(Command::new("make")
    //     .arg("install")
    //     .current_dir(&src_build_path));

    src_path.clone()
}

/*
/// Downloads a file.
#[cfg(feature = "compile_nc")]
fn download(url_zipfile: &str, local_file: &Path) {
    if !local_file.exists() {
        println!("cargo:warning=downloading...");
        let mut resp = get(url_zipfile).expect("request failed");
        let mut out = File::create(&local_file).expect("failed to create `out` file");
        io::copy(&mut resp, &mut out).expect("failed to copy content");
    } else {
        println!("cargo:warning=already downloaded!...");
    }
}
*/

/// Runs a `Command`.
#[cfg(feature = "compile_nc")]
fn run(command: &mut Command) {
    println!("cargo:warning=Running: {:?}", command);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                panic!("`{:?}` failed: {}", command, status);
            }
        }
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}
