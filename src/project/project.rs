// externals
use filesystem;
use git2::Repository;
use semver::Version;
use std::path;
// locals
use super::package_manager::{identify_dir, PackageManager};

pub struct Project<'a> {
    directory: path::PathBuf,
    package_manager: &'a PackageManager,
}

impl Project<'_> {
    pub fn path(&self) -> &path::PathBuf {
        &self.directory
    }

    pub fn language_name(&self) -> &'static str {
        self.package_manager.language_name()
    }

    pub fn bump_major(&self) -> semver::Version {
        // get the current version
        let mut next_version = self.current_version().unwrap();

        // increment it by one major version
        next_version.increment_major();
        self.package_manager.major(&next_version);

        // return it to the caller
        return next_version;
    }

    pub fn bump_minor(&self) -> semver::Version {
        // get the current version
        let mut next_version = self.current_version().unwrap();

        // increment it by one major version
        next_version.increment_minor();
        self.package_manager.minor(&next_version);

        // return it to the caller
        return next_version;
    }

    pub fn bump_patch(&self) -> semver::Version {
        // get the current version
        let mut next_version = self.current_version().unwrap();

        // increment it by one major version
        next_version.increment_patch();
        self.package_manager.patch(&next_version);

        // return it to the caller
        return next_version;
    }

    fn current_version(&self) -> Result<semver::Version, &'static str> {
        // get the list of tags for the current repository
        let current_repo = match Repository::open(self.path()) {
            Err(_) => return Err("Should get this."),
            Ok(repo) => repo,
        };

        // grab the tags from the repo
        let tags = match current_repo.tag_names(None) {
            Err(_) => return Err("Error looking up tags"),
            Ok(strings) => strings,
        };

        // if we dont have any versions
        if tags.len() == 0 {
            // then we're at v0.0.0
            match Version::parse("0.0.0") {
                Ok(version) => return Ok(version),
                Err(_) => return Err("Shouldn't get here"),
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
                None => Err("hello"),
            };
        }
    }
}

pub fn new(fs: impl filesystem::FileSystem, dir: path::PathBuf, mgr: &PackageManager) -> Project {
    return Project {
        directory: dir,
        package_manager: mgr,
    };
}

// getters

pub fn find<'a>(fs: impl filesystem::FileSystem) -> Result<Project<'a>, &'static str> {
    // find the directory with the git repo
    let dir = match find_dir(&fs) {
        Err(msg) => return Err(msg),
        Ok(d) => d,
    };

    // grab the appropriate manager
    let manager = match identify_dir(&fs, &dir) {
        Err(msg) => return Err(msg),
        Ok(mgr) => mgr,
    };

    // return a new instance of the project
    Ok(new(fs, dir, manager))
}

fn find_dir(fs: &impl filesystem::FileSystem) -> Result<path::PathBuf, &'static str> {
    // find the current directory
    let cwd = match fs.current_dir() {
        Err(_) => return Err("Could not find current working dir"),
        Ok(dir) => dir,
    };

    // look for the directory
    return find_dir_walk(&cwd);
}

fn find_dir_walk(dir: &path::PathBuf) -> Result<path::PathBuf, &'static str> {
    // if we have a directory called .git here
    if dir.join(".git").is_dir() {
        // we found the repo, we're done
        return Ok(dir.clone());
    }

    // grab the parent directory
    let parent = match dir.parent() {
        None => return Err("Could not find git repo"),
        Some(par) => par,
    };

    // check if the parent is the repo
    return find_dir_walk(&parent.to_path_buf());
}
