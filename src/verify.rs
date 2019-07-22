use crate::error::Error;
use clap::ArgMatches;
use std::process::Command;

pub fn all(matches: &ArgMatches) -> Result<(), Error> {
    test(matches)?;
    build(matches)?;
    doc(matches)?;
    fmt(matches)?;
    Ok(())
}

pub fn test(_matches: &ArgMatches) -> Result<(), Error> {
    run("cargo", &["test"])
}

pub fn build(_matches: &ArgMatches) -> Result<(), Error> {
    run("cargo", &["build"])
}

pub fn doc(_matches: &ArgMatches) -> Result<(), Error> {
    run("cargo", &["doc"])
}

pub fn fmt(_matches: &ArgMatches) -> Result<(), Error> {
    run("cargo", &["fmt", "--", "--check"])
}

fn run(cmd: &str, args: &[&str]) -> Result<(), Error> {
    println!("=== Running {:?} {:?}", cmd, args);

    Command::new(cmd).args(args).status().map(|s| {
        assert!(s.success());
        ()
    })?;

    Ok(())
}
