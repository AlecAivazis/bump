use semver::Version;

pub trait PackageManager {
    // these need to be implemented by each language
    fn language_name(&self) -> String;
    fn major(&self);
    fn minor(&self);
    fn patch(&self);
    fn pre(&self);

    fn current_version(&self) -> semver::Version {
        return Version::parse("1.2.3").unwrap();
    }
    fn next_major(&self) -> semver::Version {
        // get the current version
        let mut version = self.current_version();
        // increment it by one major version
        version.increment_major();

        // pass the modified version on
        return version;
    }

    fn next_minor(&self) -> semver::Version {
        // get the current version
        let mut version = self.current_version();
        // increment it by one major version
        version.increment_minor();

        // pass the modified version on
        return version;
    }

    fn next_patch(&self) -> semver::Version {
        // get the current version
        let mut version = self.current_version();
        // increment it by one major version
        version.increment_patch();

        // pass the modified version on
        return version;
    }
}
