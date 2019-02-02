// externals
use git2;
// locals
use crate::project;

// cargo::PackageManager knows how to bump cargo packages
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "cargo"
    }
    fn major(&self, _repo: &git2::Repository, _version: &semver::Version) -> Result<(), String> {
        Err(String::from("Cargo is not yet supported."))
    }
    fn minor(&self, _repo: &git2::Repository, _version: &semver::Version) -> Result<(), String> {
        Err(String::from("Cargo is not yet supported."))
    }
    fn patch(&self, _repo: &git2::Repository, _version: &semver::Version) -> Result<(), String> {
        Err(String::from("Cargo is not yet supported."))
    }
}
