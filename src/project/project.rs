// externals
use filesystem;
use git2::Repository;
use semver::Version;
use std::path;
// locals
use super::package_manager::{identify_dir, PackageManager};

pub struct Project<'a> {
    directory: path::PathBuf,
    repository: git2::Repository,
    package_manager: &'a PackageManager,
}

impl Project<'_> {
    pub fn path(&self) -> &path::PathBuf {
        &self.directory
    }

    pub fn language_name(&self) -> &'static str {
        self.package_manager.language_name()
    }

    pub fn bump_major(&self) -> Result<semver::Version, String> {
        // get the current version
        let mut next_version = self.current_version().unwrap();

        // increment it by one major version
        next_version.increment_major();
        match self.package_manager.major(&self.repository, &next_version) {
            Ok(_) => (),
            Err(msg) => return Err(msg),
        };

        // return it to the caller
        return Ok(next_version);
    }

    pub fn bump_minor(&self) -> Result<semver::Version, String> {
        // get the current version
        let mut next_version = self.current_version().unwrap();

        // increment it by one minor version
        next_version.increment_minor();
        match self.package_manager.minor(&self.repository, &next_version) {
            Ok(_) => (),
            Err(msg) => return Err(msg),
        };

        // return it to the caller
        return Ok(next_version);
    }

    pub fn bump_patch(&self) -> Result<semver::Version, String> {
        // get the current version
        let mut next_version = self.current_version().unwrap();

        // increment it by one patch version
        next_version.increment_patch();
        match self.package_manager.patch(&self.repository, &next_version) {
            Ok(_) => (),
            Err(msg) => return Err(msg),
        };

        // return it to the caller
        return Ok(next_version);
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
                        Some(tag) => {
                            let name = if tag.to_string().chars().nth(0).unwrap() == 'v' {
                                &tag[1..]
                            } else {
                                tag
                            };
                            println!("found tag named {}", name);

                            match Version::parse(name) {
                                // the tag is valid semver
                                Ok(version) => {
                                    // add the parsed version to the list
                                    acc.push(version);
                                    acc
                                }
                                // if its not a valid semver tag then don't include it
                                Err(_) => acc,
                            }
                        }
                        // if the tag is not utf-8 then dont include it
                        None => acc,
                    }
                });
            // make sure we get the highest version first
            ordered_tags.sort_by(|a, b| b.cmp(a));

            // the first entry in the ordered list of tags is the current semantic version
            return match ordered_tags.get(0) {
                Some(tag) => Ok(tag.clone()),
                None => Err("hello"),
            };
        }
    }
}

// getters

pub fn find<'a>(fs: impl filesystem::FileSystem) -> Result<Project<'a>, &'static str> {
    // find the directory with the git repo
    let dir = match find_dir(&fs) {
        Err(msg) => return Err(msg),
        Ok(d) => d,
    };

    // and the repository associated with it
    let repo = match Repository::open(&dir) {
        Err(_) => return Err("Directory was not a git repo??"),
        Ok(r) => r,
    };

    // grab the appropriate manager
    let manager = identify_dir(&fs, &dir);

    // return a new instance of the project
    Ok(Project {
        repository: repo,
        directory: dir,
        package_manager: manager,
    })
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
