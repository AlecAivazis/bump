// locals
use crate::project;

// cargo::Project knows how to bump cargo packages
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "cargo"
    }
    fn major(&self, version: &semver::Version) {
        println!("{}", version)
    }
    fn minor(&self, _version: &semver::Version) {}
    fn patch(&self, _version: &semver::Version) {}
}
