// locals
use super::version;

// go::PackageManager knows how to bump go packages
pub struct PackageManager;

impl version::PackageManager for PackageManager {
    fn language_name(&self) -> String {
        String::from("cargo")
    }
    fn major(&self) {
        // compute the next major version
        let next_version = self.next_major();

        println!("{}", next_version)
    }
    fn minor(&self) {}
    fn patch(&self) {}
    fn pre(&self) {}
}
