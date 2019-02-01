// externals
use structopt::StructOpt;
#[macro_use]
extern crate clap;
use std::env;
// local module definition
mod go;
mod identify;
mod node;

pub trait PackageManager {
    fn language_name(&self) -> String;
    fn major(&self);
    fn minor(&self);
    fn patch(&self);
    fn pre(&self);
}

fn main() {
    // grab the arguments from the command line
    let args = BumpCli::from_args();

    match env::current_dir() {
        Err(_) => panic!("Could not find current working dir"),
        Ok(cwd) => {
            // grab package manager for the current project
            match identify::identify_project(cwd, filesystem::OsFileSystem::new()) {
                Err(msg) => println!("Encountered error: {}", msg),
                Ok(mgr) => {
                    println!("Identified project as {}", mgr.language_name());
                    println!("Bumping package one {} version...", args.amount);

                    // perform the appropriate bump depending on the version
                    match args.amount {
                        BumpAmount::Major => mgr.major(),
                        BumpAmount::Minor => mgr.minor(),
                        BumpAmount::Patch => mgr.patch(),
                        BumpAmount::Pre => mgr.pre(),
                    };
                }
            };
        }
    }
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
        Pre,
    }
}
