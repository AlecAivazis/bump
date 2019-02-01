// locals
use super::node;

pub fn identify_project() -> Result<impl super::PackageManager, &'static str> {
    Ok(&node::PackageManager {})
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
