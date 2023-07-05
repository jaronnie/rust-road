mod command;

use clap::{Command};

fn main() {
    let matches = Command::new("cli")
        .version("1.0")
        .author("jaronnie")
        .about("A cli tool using rust to learn clap")
        .subcommand(Command::new("install").about("Install something"))
        .subcommand(Command::new("activate").about("Activate something"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("install") {
        let command = command::install::InstallCommand;
        command.execute();
    } else if let Some(_matches) = matches.subcommand_matches("activate") {
        let command = command::activate::ActivateCommand;
        command.execute();
    }
}
