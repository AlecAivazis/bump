// local imports
mod package_manager;
mod project;

// rexport under a flat module
pub use package_manager::PackageManager;
pub use project::find;
pub use project::new;
pub use project::Project;
