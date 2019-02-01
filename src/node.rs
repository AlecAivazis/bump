// locals
use super::version;

// node::PackageManager knows how to bump node packages
pub struct PackageManager;

impl version::PackageManager for PackageManager {
    fn language_name(&self) -> String {
        "Node".to_string()
    }
    fn major(&self) {
        //
    }
    fn minor(&self) {}
    fn patch(&self) {}
    fn pre(&self) {}
}
