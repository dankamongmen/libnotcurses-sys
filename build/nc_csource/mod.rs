//!
//!

#![allow(dead_code)]

// MAYBE
// - add set_install_path method
// - add install method

use std::{
    env::{set_var, var},
    fs::{create_dir_all, remove_dir_all},
    io::{ErrorKind, Result as IoResult},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[cfg(feature = "nc_compile")]
extern crate cc;

/// The URL of the repository of the notcurses C source.
pub const C_SRC_REPO: &str = "https://github.com/dankamongmen/notcurses";

/// The base name for the local notcurses C source directory & compressed file.
pub const C_SRC_BASENAME: &str = "notcurses4libnotcurses-sys";

/// An abstraction over the original source of notcurses, in C.
///
/// Used to:
/// - compile the source.
/// - obtain the source from the repository.
///     - by
/// - vendor the source inside the crate, for publising.
#[derive(Clone, Debug, Default)]
pub struct NcCSource {
    /// The desired notcurses version.
    version: String,

    /// The base path.
    ///
    /// `$OUT_DIR` by default.
    root_path: PathBuf,

    /// The path to the notcurses C source code directory.
    source_path: PathBuf,

    /// The name of the notcurses C source code compressed file.
    compressed_source_file: String,

    /// The path to the notcurses C source code compressed file.
    compressed_source_path: PathBuf,

    /// The path to the vendored directory to be embedded in the crate.
    vendored_path: PathBuf,

    /// The path to the vendored source file to be embedded in the crate.
    vendored_compressed_source_path: PathBuf,

    /// The path to the notcurses C headers.
    headers_path: PathBuf,

    /// The path to the notcurses C build path.
    build_path: PathBuf,
}

/// Methods directly associated with features.
impl NcCSource {
    /// Embed the C source in the crate.
    ///
    /// Will be called if the "nc_vendor" feature is enabled.
    pub fn vendor(&self) {
        println!("cargo:warning=Vendoring…");

        // clone the repository optimizing size
        self.clone_repo(true);

        // compress the repository into a new file
        self.compress_source();

        // make sure the vendored directory exists
        create_dir_all(&self.vendored_path).expect("couldn't create 'vendored/' directory");

        // copy the compressed file to the vendored directory.
        Self::run(
            Command::new("cp")
                .arg(&self.compressed_source_path)
                .arg(&self.vendored_path)
                .current_dir(&self.root_path),
        );
    }

    /// Makes sure the source code will not be vendored.
    ///
    /// Will be called if the "nc_vendor" feature is NOT enabled.
    pub fn dont_vendor(&self) {
        Self::rm(&self.vendored_path)
            .unwrap_or_else(|_| panic!["rm -rf vendored: {:?}", self.vendored_path]);
    }

    /// Intended for compiling the `notcurses` C library in of docs.rs.
    // WIP
    pub fn compile(&self) {
        println!("cargo:warning=Compiling…");
        self.decompress_source(&self.vendored_compressed_source_path);

        // NOTE: it assumes dependencies are installed

        // prepare the building directory
        create_dir_all(&self.build_path).expect("couldn't create 'build/' directory");

        set_var("PKG_CONFIG_PATH", &self.build_path);
        set_var("CPATH", &self.build_path.join("include/notcurses"));

        // compile notcurses

        // let docs_rs = std::env::var("DOCS_RS").unwrap_or_else(|_| "".to_string()) == "1";
        // let use_libdeflate = if docs_rs { "-DUSE_DEFLATE=off" } else { "" };

        Self::run(
            Command::new("cmake")
                // .arg("-DCMAKE_INSTALL_PREFIX=/usr/local/") // (disabled install)
                .arg("-DUSE_DOCTEST=off")
                .arg("-DUSE_PANDOC=off")
                // .arg(use_libdeflate)
                .arg("..")
                .current_dir(&self.build_path),
        );

        Self::run(
            Command::new("make")
                .arg(format!("-j{}", var("NUM_JOBS").expect("ERR: NUM_JOBS")))
                .current_dir(&self.build_path),
        );
    }

    // /// MAYBE install notcurses
    // pub fn install(&self) {
    //     Self::run(Command::new("make")
    //         .arg("install")
    //         .current_dir(&self.build_path));
    // }
}

impl NcCSource {
    /// Returns a new `NcCSource` for the specified version.
    ///
    /// By default it uses the $root_path path as the base path.
    pub fn new(version: &str) -> Self {
        let mut self0 = NcCSource {
            compressed_source_file: format!["{}.tar.xz", C_SRC_BASENAME],
            ..Default::default()
        };
        self0.set_root_path(PathBuf::from(var("OUT_DIR").expect("ERR: OUT_DIR")));

        let vendored_path =
            PathBuf::from(var("CARGO_MANIFEST_DIR").expect("ERR: CARGO_MANIFEST_DIR"))
                .join("build/vendored");
        let vendored_compressed_source_path = vendored_path.join(&self0.compressed_source_file);

        Self {
            version: version.into(),
            vendored_path,
            vendored_compressed_source_path,
            ..self0
        }
    }

    /// Gets the version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Sets the version.
    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    /// Returns the base path.
    pub fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    /// Sets the base path, and recalculates the derivated paths.
    pub fn set_root_path(&mut self, root_path: PathBuf) {
        // println!("cargo:warning=Setting root_path={:?}", root_path);
        self.root_path = root_path;
        self.source_path = self.root_path.join(C_SRC_BASENAME);
        self.compressed_source_path = self.root_path.join(&self.compressed_source_file);
        self.headers_path = self.source_path.join("include");
        self.build_path = self.source_path.join("build");
    }

    /// Returns the path to the headers for inclusing with the format "-I$PATH".
    pub fn headers_include_string(&self) -> String {
        format!["-I{}", self.headers_path.to_string_lossy()]
    }

    /// Clones the C source repository.
    ///
    /// If `optimize_size` is true, only the tagged branch will be cloned, and
    /// all the unnecessary files for compilation will be deleted. Otherwise
    /// the full repository will be cloned and all files will be maintained.
    pub fn clone_repo(&self, optimize_size: bool) {
        if optimize_size {
            println!("cargo:warning=Cloning the repository (size optimize)…");
        } else {
            println!("cargo:warning=Cloning the full repository…");
        }

        // make sure the target path doesn't already exist.
        Self::rm(&self.source_path)
            .unwrap_or_else(|_| panic!["rm -rf source_path: {:?}", self.source_path]);

        // clone the branch we want from the repo.
        let mut git_cmd = Command::new("git");
        if optimize_size {
            git_cmd
                .arg("clone")
                .arg("--depth")
                .arg("1")
                .arg("--branch")
                .arg(&format!["v{}", self.version])
                .arg(C_SRC_REPO)
                .arg(&self.source_path)
                .stderr(Stdio::null())
                .current_dir(&self.root_path)
        } else {
            git_cmd
                .arg("clone")
                .arg(C_SRC_REPO)
                .arg(&self.source_path)
                .stderr(Stdio::null())
                .current_dir(&self.root_path)
        };
        Self::run(&mut git_cmd);

        // remove unnecessary directories to save space.
        if optimize_size {
            let delete_files = ["build", "data", "doc", ".git", "python", "cffi"];
            println!("cargo:warning=deleting files: {:?}...", &delete_files);
            for file in delete_files {
                let file_path = self.source_path.join(file);
                Self::rm(&file_path).unwrap_or_else(|_| panic!["rm -rf {:?}", file_path]);
            }
        }
    }

    /// Compresses the C source directory.
    ///
    /// Uses [xz compression](https://en.wikipedia.org/wiki/XZ_Utils).
    pub fn compress_source(&self) {
        println!("cargo:warning=Compressing…");

        set_var("XZ_OPT", "-e9");
        Self::run(
            Command::new("tar")
                .arg("cJf")
                .arg(&self.compressed_source_file)
                .arg("-C")
                .arg(&self.root_path)
                .arg(C_SRC_BASENAME)
                .current_dir(&self.root_path),
        );
    }

    /// Decompresses the vendored C source directory.
    pub fn decompress_source<P: AsRef<Path>>(&self, file_path: P) {
        println!("cargo:warning=Decompressing…");
        Self::run(
            Command::new("tar")
                .arg("xJf")
                .arg(file_path.as_ref())
                .arg("-C")
                .arg(&self.root_path)
                .current_dir(&self.root_path),
        );
    }

    /// Install the dependencies needed to compile notcurses.
    ///
    /// NOTE: for now it assumes "apt" is available (debian/ubuntu based distro).
    /// This works well for docs.rs but it may ask for password in other systems.
    pub fn install_dependencies(&self) {
        Self::run(
            Command::new("apt")
                .arg("install")
                .arg("-y")
                .arg("libunistring-dev")
                .arg("libdeflate-dev")
                .arg("doctest-dev"),
        );
    }
}

// private utility functions
impl NcCSource {
    /// Runs a `Command`.
    fn run(command: &mut Command) {
        println!(
            "cargo:warning=Running: {:?}",
            format!["{:?}", command].replace("\"", "")
        );
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

    /// Deletes a path. Including full directories.
    ///
    /// Wont return an error if the target path doesn't exist.
    fn rm<P: AsRef<Path>>(path: P) -> IoResult<()> {
        match remove_dir_all(path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Ok(()),
                _ => Err(e),
            },
        }
    }

    pub fn print_debug(&self) {
        println!("cargo:warning=root_: {:?}", self.root_path);
        println!("cargo:warning=sourc: {:?}", self.source_path);
        println!("cargo:warning=cfile: {:?}", self.compressed_source_file);
        println!("cargo:warning=cpath: {:?}", self.compressed_source_path);
        println!("cargo:warning=vpath: {:?}", self.vendored_path);
        println!(
            "cargo:warning=vcomp: {:?}",
            self.vendored_compressed_source_path
        );
        println!("cargo:warning=heade: {:?}", self.headers_path);
        println!("cargo:warning=build:{:?}", self.build_path);
    }
}
