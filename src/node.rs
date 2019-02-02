// externals
use std::process::Command;
use std::str;
// locals
use super::project;

pub struct PackageManager;

impl PackageManager {
    fn bump_version(&self, which: &str) {
        // execute the command
        match Command::new("npm").args(&["version", which]).output() {
            Ok(_) => (),
            Err(res) => println!("Something went wrong: {}", res),
        };
    }
}

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "node"
    }

    fn major(&self, _version: &semver::Version) {
        self.bump_version("major")
    }

    fn minor(&self, _version: &semver::Version) {
        self.bump_version("minor")
    }

    fn patch(&self, _version: &semver::Version) {
        self.bump_version("patch")
    }
}
