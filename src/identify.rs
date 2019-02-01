// externals
// locals
use super::javascript;

pub fn identify_project() -> Result<impl super::PackageManager, &'static str> {
    Ok(&javascript::PackageManager {})
}
