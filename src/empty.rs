// externals
use git2;
// locals
use super::project;

// empty::PackageManager knows how to bump packages that aren't in a langauge we understand
pub struct PackageManager;

impl project::PackageManager for PackageManager {
    fn language_name(&self) -> &'static str {
        "empty"
    }

    fn major(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String> {
        self.create_tag(
            repo,
            format!("{}.{}.{}", version.major, version.minor, version.patch),
        )
    }

    fn minor(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String> {
        self.create_tag(
            repo,
            format!("{}.{}.{}", version.major, version.minor, version.patch),
        )
    }

    fn patch(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String> {
        self.create_tag(
            repo,
            format!("{}.{}.{}", version.major, version.minor, version.patch),
        )
    }
}
