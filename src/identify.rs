// externals
use filesystem;
use std::path;
// locals
use super::go;
use super::node;

// identify_project should take in a directory path and a file system to use and figure
// out which package manage we want to use
pub fn identify_project(
    cwd: path::PathBuf,
    fs: impl filesystem::FileSystem,
) -> Result<&'static super::PackageManager, &'static str> {
    // if there is a node package manifest
    match fs.read_file(cwd.join("package.json")) {
        Err(_) => (),
        Ok(_) => {
            return Ok(&node::PackageManager {});
        }
    }

    // if there is a go module file
    match fs.read_file(cwd.join("go.mod")) {
        Err(_) => (),
        Ok(_) => {
            return Ok(&go::PackageManager {});
        }
    }

    return Err("Could not identify project");
}

#[cfg(test)]
mod tests {
    use filesystem::FileSystem;
    use std::path;

    #[test]
    fn identify_node() {
        let cwd = "home";
        // create an in memory filesystem we will test again
        let mem_fs = filesystem::FakeFileSystem::new();

        // make sure that the cwd is a directory in the filesystem
        match mem_fs.create_dir(cwd) {
            Err(msg) => panic!(msg),
            _ => (),
        }

        // create a package.json file underneath the cwd
        match mem_fs.create_file("home/package.json", "{\"version\": \"1.0.1\"}") {
            Err(msg) => panic!(msg),
            _ => (),
        }

        // identify the project
        match crate::identify::identify_project(path::PathBuf::from(cwd), mem_fs) {
            Ok(mgr) => assert_eq!("Node", mgr.language_name()),
            _ => panic!("Could not idenfity node package"),
        }
    }

    #[test]
    fn identify_go() {
        let cwd = "home";
        // create an in memory filesystem we will test again
        let mem_fs = filesystem::FakeFileSystem::new();

        // make sure that the cwd is a directory in the filesystem
        match mem_fs.create_dir(cwd) {
            Err(msg) => panic!(msg),
            _ => (),
        }

        // create a package.json file underneath the cwd
        match mem_fs.create_file("home/go.mod", "content") {
            Err(msg) => panic!(msg),
            _ => (),
        }

        // identify the project
        match crate::identify::identify_project(path::PathBuf::from(r"home"), mem_fs) {
            Ok(mgr) => assert_eq!("Go", mgr.language_name()),
            _ => panic!("Could not idenfity go package"),
        }
    }
}
