mod error;
mod verify;

use crate::error::Error;
use clap::{App, SubCommand};

fn main() -> Result<(), Error> {
    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verify aspects of the workspace or package.")
                .subcommand(SubCommand::with_name("all").about("Run all verifications."))
                .subcommand(SubCommand::with_name("test").about("cargo test"))
                .subcommand(SubCommand::with_name("build").about("cargo build"))
                .subcommand(SubCommand::with_name("doc").about("cargo doc"))
                .subcommand(SubCommand::with_name("fmt").about("cargo fmt verification")),
        )
        .subcommand(
            SubCommand::with_name("git-hooks")
                .about("Manage git-hooks.")
                .subcommand(SubCommand::with_name("install").about("Install git hooks.")),
        );

    let matches = app.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        if let Some(matches) = matches.subcommand_matches("all") {
            verify::all(matches)
        } else if let Some(matches) = matches.subcommand_matches("test") {
            verify::test(matches)
        } else if let Some(matches) = matches.subcommand_matches("build") {
            verify::build(matches)
        } else if let Some(matches) = matches.subcommand_matches("doc") {
            verify::doc(matches)
        } else if let Some(matches) = matches.subcommand_matches("fmt") {
            verify::fmt(matches)
        } else {
            verify::all(matches)
        }
    } else if let Some(matches) = matches.subcommand_matches("git-hooks") {
        if let Some(matches) = matches.subcommand_matches("install") {
            unimplemented!("git-hooks install {:?}", matches);
        } else {
            app.print_help()?;
            Ok(())
        }
    } else {
        app.print_help()?;
        Ok(())
    }
}
