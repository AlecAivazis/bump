// locals

pub struct PackageManager;

impl super::PackageManager for &PackageManager {
    fn language_name(&self) -> String {
        "JavaScript".to_string()
    }
    fn major(&self) {}
    fn minor(&self) {}
    fn patch(&self) {}
    fn pre(&self) {}
}
