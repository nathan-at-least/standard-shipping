mod verify;

use clap::{App, Error, SubCommand};

fn main() -> Result<(), Error> {
    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verify aspects of the workspace or package.")
                .subcommand(
                    SubCommand::with_name("all")
                        .about("Run all verifications.")
                )
        );

    let matches = app.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        if let Some(matches) = matches.subcommand_matches("all") {
            verify::all(matches)
        } else {
            verify::all(matches)
        }
    } else {
        app.print_help()?;
        Ok(())
    }
}
