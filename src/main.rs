// externals
use structopt::StructOpt;
#[macro_use]
extern crate clap;
// local module definition
mod cargo;
mod empty;
mod go;
mod node;
mod project;

fn main() {
    // grab the arguments from the command line
    let args = BumpCli::from_args();

    // find the current repository
    let current_project = match project::find(filesystem::OsFileSystem::new()) {
        Err(msg) => panic!(msg),
        Ok(r) => r,
    };

    println!(
        "✅  Identified {} project",
        current_project.language_name().to_lowercase()
    );
    println!(
        "✅  Bumping package up a {} version",
        args.amount.to_string().to_lowercase()
    );

    // perform the appropriate bump depending on the version
    let next_version = match args.amount {
        BumpAmount::Major => current_project.bump_major(),
        BumpAmount::Minor => current_project.bump_minor(),
        BumpAmount::Patch => current_project.bump_patch(),
    };

    println!("✅  Cutting tag {}", next_version.unwrap());
}

#[derive(Debug, StructOpt)]
struct BumpCli {
    /// can be one of "major", "minor", "patch"
    amount: BumpAmount,
}

clap::arg_enum! {
    #[derive(Debug)]
    enum BumpAmount {
        Major,
        Minor,
        Patch,
    }
}
