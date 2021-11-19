//! shared module

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

#[allow(dead_code)]
fn main() {
    println!("This is just a shared module used by the examples.");
}

/// Returns a path relative to the project's root.
pub fn project_root_path(path: &Path) -> io::Result<PathBuf> {
    let current_path = env::current_dir()?;
    let mut root_path = current_path.clone();

    for p in current_path.as_path().ancestors() {
        let has_cargo = fs::read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == *"Cargo.toml");
        if has_cargo {
            return Ok(root_path.join(path));
        } else {
            root_path.pop();
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}

/// Like [`project_root_path`] but accepts `&str` and returns `String`.
///
/// In case of an error the returned string will be empty.
pub fn project_root_path_string(path: &str) -> String {
    project_root_path(Path::new(path)).map_or("".into(), |p| p.to_str().unwrap().to_owned())
}
