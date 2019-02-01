// externals
use structopt::StructOpt;
#[macro_use]
extern crate clap;
// local module definition
mod identify;
mod javascript;

pub trait PackageManager {
    fn language_name(&self) -> String;
    fn major(&self);
    fn minor(&self);
    fn patch(&self);
    fn pre(&self);
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

#[derive(Debug, StructOpt)]
struct BumpCli {
    /// can be one of "major", "minor", "patch"
    amount: BumpAmount,
}

fn main() {
    // grab the arguments from the command line
    let args = BumpCli::from_args();

    // grab package manager for the current project
    match identify::identify_project() {
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
