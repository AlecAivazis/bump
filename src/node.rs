// externals
use std::error::Error;
use std::process::Command;
use std::str;
// locals
use super::project;

pub struct PackageManager;

impl PackageManager {
    fn bump_version(&self, which: &str) -> Result<(), String> {
        // execute the command
        match Command::new("npm").args(&["version", which]).output() {
            Ok(_) => Ok(()),
            Err(res) => Err(res.to_string()),
        }
    }
}

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "node"
    }

    fn major(&self, _version: &semver::Version) -> Result<(), String> {
        self.bump_version("major")
    }

    fn minor(&self, _version: &semver::Version) -> Result<(), String> {
        self.bump_version("minor")
    }

    fn patch(&self, _version: &semver::Version) -> Result<(), String> {
        self.bump_version("patch")
    }
}
