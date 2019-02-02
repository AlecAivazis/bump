// external imports
use std::path;
// local imports
use crate::cargo;
use crate::go;
use crate::node;

// PackageManager represents the core actor in bump. it knows where it lives and how to
// bump versions appropriately.
pub trait PackageManager {
    // these need to be implemented by each language
    fn language_name(&self) -> &'static str;
    fn major(&self, version: &semver::Version) -> Result<(), String>;
    fn minor(&self, version: &semver::Version) -> Result<(), String>;
    fn patch(&self, version: &semver::Version) -> Result<(), String>;
}

pub fn identify_dir<'a>(
    fs: &impl filesystem::FileSystem,
    dir: &path::PathBuf,
) -> Result<&'a PackageManager, &'static str> {
    // if there is a node package manifest
    if fs.read_file(dir.join("package.json")).is_ok() {
        return Ok(&node::PackageManager {});
    }

    // if there is a go module file
    if fs.read_file(dir.join("go.mod")).is_ok() {
        return Ok(&go::PackageManager {});
    }

    // cargo file
    if fs.read_file(dir.join("cargo.toml")).is_ok() {
        return Ok(&cargo::PackageManager {});
    }

    return Err("Could not identify project");
}
