// externals
use git2;
// locals
use super::project;

// go::PackageManager knows how to bump go packages
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "go"
    }

    fn major(&self, repo: &git2::Repository, _version: &semver::Version) -> Result<(), String> {
        Err(String::from("Major bumps are not yet supported in go."))
    }

    fn minor(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String> {
        // we have to cut the tag in the form of vX.X.X instead of the usual X.X.X
        self.create_tag(
            repo,
            format!("v{}.{}.{}", version.major, version.minor, version.patch),
        )
    }

    fn patch(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String> {
        // we have to cut the tag in the form of vX.X.X instead of the usual X.X.X
        self.create_tag(
            repo,
            format!("v{}.{}.{}", version.major, version.minor, version.patch),
        )
    }
}
