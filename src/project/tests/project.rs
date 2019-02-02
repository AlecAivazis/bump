#[cfg(test)]
mod project_tests {
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
        match crate::project::identify_project(path::PathBuf::from(cwd), mem_fs) {
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
        match crate::project::identify_project(path::PathBuf::from(r"home"), mem_fs) {
            Ok(mgr) => assert_eq!("Go", mgr.language_name()),
            _ => panic!("Could not idenfity go package"),
        }
    }

    #[test]
    fn find_repoAbove() {
        // create an in memory filesystem we will test again
        let mem_fs = filesystem::FakeFileSystem::new();
        // add a .git directory (the indicator of a git repo) to a known location
        mem_fs.create_dir("/.git");
    }

    #[test]
    fn find_currentRepo() {
        // create an in memory filesystem we will test again
        let mem_fs = filesystem::FakeFileSystem::new();
        // set the current working
    }
}
