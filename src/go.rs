// locals
use super::project;

// go::PackageManager knows how to bump go packages
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "go"
    }
    fn major(&self, _version: &semver::Version) -> Result<(), String> {
        Ok(())
    }
    fn minor(&self, _version: &semver::Version) -> Result<(), String> {
        Ok(())
    }
    fn patch(&self, _version: &semver::Version) -> Result<(), String> {
        Ok(())
    }
}
