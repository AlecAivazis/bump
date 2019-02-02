// external imports
use std::path;
// local imports
use crate::cargo;
use crate::empty;
use crate::go;
use crate::node;

// PackageManager represents the core actor in bump. it knows where it lives and how to
// bump versions appropriately.
pub trait PackageManager {
    // these need to be implemented by each language
    fn language_name(&self) -> &'static str;
    fn major(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String>;
    fn minor(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String>;
    fn patch(&self, repo: &git2::Repository, version: &semver::Version) -> Result<(), String>;

    fn create_tag(&self, repo: &git2::Repository, tag_name: String) -> Result<(), String> {
        // grab the current commit
        let head = match repo.head() {
            Ok(reference) => reference,
            Err(msg) => return Err(msg.to_string()),
        };
        let head_obj = repo
            .find_object(head.target().unwrap(), Some(git2::ObjectType::Commit))
            .unwrap();

        // create a tag in the repo
        match repo.tag_lightweight(&tag_name, &head_obj, false) {
            Ok(_) => (),
            Err(msg) => return Err(msg.to_string()),
        };

        // notify the user
        println!("✅  Created tag {}", tag_name);

        // grab the head
        Ok(())
    }
}

pub fn identify_dir<'a>(
    fs: &impl filesystem::FileSystem,
    dir: &path::PathBuf,
) -> &'a PackageManager {
    // if there is a node package manifest
    if fs.read_file(dir.join("package.json")).is_ok() {
        return &node::PackageManager {};
    }

    // if there is a go module file
    if fs.read_file(dir.join("go.mod")).is_ok() {
        return &go::PackageManager {};
    }

    // cargo file
    if fs.read_file(dir.join("cargo.toml")).is_ok() {
        return &cargo::PackageManager {};
    }

    return &empty::PackageManager {};
}
