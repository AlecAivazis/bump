// locals
use super::version;

// go::PackageManager knows how to bump go packages
pub struct PackageManager;

impl version::PackageManager for PackageManager {
    fn language_name(&self) -> String {
        String::from("Rust")
    }
    fn major(&self) {
        // compute the next major version
        let nextVersion = self.next_major();

        println!("{}", nextVersion)
    }
    fn minor(&self) {}
    fn patch(&self) {}
    fn pre(&self) {}
}
