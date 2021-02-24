mod archive;
mod chat;
mod extensions;
mod notebook;

use archive::*;
use chat::*;
use docopt::Docopt;
use notebook::*;
use serde::Deserialize;
use urbit_http_api::{
    create_new_ship_config_file, ship_interface_from_config, ship_interface_from_local_config,
    ShipInterface,
};

const ASCII_TITLE: &'static str = r#"
  _    _      _     _ _      _____            _             _                       _     _
 | |  | |    | |   (_) |    / ____|          | |           | |       /\            | |   (_)
 | |  | |_ __| |__  _| |_  | |     ___  _ __ | |_ ___ _ __ | |_     /  \   _ __ ___| |__  ___   _____ _ __
 | |  | | '__| '_ \| | __| | |    / _ \| '_ \| __/ _ \ '_ \| __|   / /\ \ | '__/ __| '_ \| \ \ / / _ \ '__|
 | |__| | |  | |_) | | |_  | |___| (_) | | | | ||  __/ | | | |_   / ____ \| | | (__| | | | |\ V /  __/ |
  \____/|_|  |_.__/|_|\__|  \_____\___/|_| |_|\__\___|_| |_|\__| /_/    \_\_|  \___|_| |_|_| \_/ \___|_|
"#;

const USAGE: &'static str = r#"
Usage:
        urbit-content-archiver chat <ship> <name> [--config=<file_path> --output=<folder_path>]
Options:
      --config=<file_path>  Specify a custom path to a YAML ship config file.
      --output=<folder_path>  Specify a custom path where the output files will be saved.

"#;

#[derive(Debug, Deserialize)]
pub struct Args {
    cmd_chat: bool,
    arg_ship: String,
    arg_name: String,
    flag_config: String,
    flag_output: String,
}

fn main() {
    // Acquire the `ShipInterface` and CLI args
    let (ship, args) = basic_setup();
    let mut channel = ship.create_channel().unwrap();

    // Print the ascii title
    println!("{}", ASCII_TITLE);

    // Chat export
    if args.cmd_chat {
        export_chat(args, &mut channel);
    }

    // Delete the channel
    channel.delete_channel();
}
/// Basic setup of generating a config file and getting a `ShipInterface` from local config.
fn basic_setup() -> (ShipInterface, Args) {
    // Read command line arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Create the archived content directory
    create_content_dir(&args);

    // If no custom config file provided
    if args.flag_config.is_empty() {
        if let Some(_) = create_new_ship_config_file() {
            println!("Ship configuration file created. Please edit it with your ship information to use the toolkit.");
            std::process::exit(0);
        }
        let ship_interface_res = ship_interface_from_local_config();
        // Error checking
        if let Some(ship) = ship_interface_res {
            return (ship, args);
        } else {
            println!("Failed to connect to Ship using information from local config.");
            std::process::exit(1);
        }
    }
    // If custom config file is provided
    else {
        if let Some(ship) = ship_interface_from_config(&args.flag_config) {
            return (ship, args);
        } else {
            println!("Failed to connect to Ship using information from custom config file.\nPlease make sure the path to the file is correct and the config is filled out properly.");
            std::process::exit(1);
        }
    }
}
