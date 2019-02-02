// locals
use crate::project;

// cargo::PackageManager knows how to bump cargo packages
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "cargo"
    }
    fn major(&self, version: &semver::Version) -> Result<(), String> {
        println!("{}", version);

        Ok(())
    }
    fn minor(&self, _version: &semver::Version) -> Result<(), String> {
        Ok(())
    }
    fn patch(&self, _version: &semver::Version) -> Result<(), String> {
        Ok(())
    }
}
