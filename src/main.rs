use clap::{App, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
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
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        if let Some(_matches) = matches.subcommand_matches("verify") {
            unimplemented!("verify all.");
        } else {
            unimplemented!("No verify subcommand.");
        }
    } else {
        unimplemented!("No top subcommand.");
    }
}
