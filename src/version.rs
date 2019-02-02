// externals
use git2::Repository;
use semver::Version;
use std::env;
use std::path;

pub trait PackageManager {
    // these need to be implemented by each language
    fn language_name(&self) -> String;
    fn major(&self);
    fn minor(&self);
    fn patch(&self);
    fn pre(&self);

    fn current_version(&self, cwd: &path::PathBuf) -> Result<semver::Version, String> {
        // get the list of tags for the current repository
        let current_repo = match Repository::open(cwd) {
            Err(_) => return Err(String::from("Current directory is not a repository")),
            Ok(repo) => repo,
        };

        // grab the tags from the repo
        let tags = match current_repo.tag_names(None) {
            Err(_) => return Err(String::from("Error looking up tags")),
            Ok(strings) => strings,
        };

        // if we dont have any versions
        if tags.len() == 0 {
            // then we're at v0.0.0
            match Version::parse("0.0.0") {
                Ok(version) => return Ok(version),
                Err(_) => return Err(String::from("Shouldn't get here")),
            }
        } else {
            // we want to sort the tags in descending order by semantic version
            let mut ordered_tags = tags
                .iter()
                // turn the string array into a list of semver compliant tags
                .fold(Vec::new(), |mut acc, tag_name| {
                    match tag_name {
                        // if we have a valid tag parse it as semver
                        Some(tag) => match Version::parse(tag) {
                            // the tag is valid semver
                            Ok(version) => {
                                // add the parsed version to the list
                                acc.push(version);
                                acc
                            }
                            // if its not a valid semver tag then don't include it
                            Err(_) => acc,
                        },
                        // if the tag is not utf-8 then dont include it
                        None => acc,
                    }
                });
            // make sure we get the highest version first
            ordered_tags.sort_by(|a, b| b.cmp(a));

            // the first entry in the ordered list of tags is the current semantic version
            // return Ok(ordered_tags.get(0).clone().unwrap());
            return match ordered_tags.get(0) {
                Some(tag) => Ok(tag.clone()),
                None => Err(String::from("hello")),
            };
        }
    }

    fn next_major(&self) -> semver::Version {
        // get the current version
        let mut version = self.current_version(&env::current_dir().unwrap()).unwrap();
        // increment it by one major version
        version.increment_major();

        // pass the modified version on
        return version;
    }

    fn next_minor(&self) -> semver::Version {
        // get the current version
        let mut version = self.current_version(&env::current_dir().unwrap()).unwrap();
        // increment it by one major version
        version.increment_minor();

        // pass the modified version on
        return version;
    }

    fn next_patch(&self) -> semver::Version {
        // get the current version
        let mut version = self.current_version(&env::current_dir().unwrap()).unwrap();
        // increment it by one major version
        version.increment_patch();

        // pass the modified version on
        return version;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn increment_major() {}

    #[test]
    fn increment_minor() {}

    #[test]
    fn increment_patch() {}
}
