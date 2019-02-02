// locals
use super::project;

// go::project knows how to bump go packages
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "go"
    }
    fn major(&self, _version: &semver::Version) {}
    fn minor(&self, _version: &semver::Version) {}
    fn patch(&self, _version: &semver::Version) {}
}
