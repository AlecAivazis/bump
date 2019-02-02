// locals
use super::version;

// go::PackageManager knows how to bump go packages
pub struct PackageManager;

impl version::PackageManager for PackageManager {
    fn language_name(&self) -> String {
        String::from("Go")
    }
    fn major(&self) {}
    fn minor(&self) {}
    fn patch(&self) {}
    fn pre(&self) {}
}
