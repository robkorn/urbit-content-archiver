mod archive;

use archive::*;
use docopt::Docopt;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use urbit_http_api::{
    create_new_ship_config_file, ship_interface_from_config, ship_interface_from_local_config,
    Channel, ShipInterface,
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

/// Exports the chat resource provided via arguments
fn export_chat(args: Args, channel: &mut Channel) {
    // Set the path where the file will be saved
    let file_name = format!("{}-{}.md", &args.arg_ship[1..], &args.arg_name);
    let file_path = format!("{}/{}", get_root_dir(&args), file_name);

    println!(
        "Requesting {}/{} chat graph from your ship...",
        &args.arg_ship, &args.arg_name
    );

    // Acquire the authored messages from the ship
    let authored_messages_res = channel
        .chat()
        .export_authored_messages(&args.arg_ship, &args.arg_name);

    // Parse the authored message, save files, and save chat messages.
    if let Ok(authored_messages) = authored_messages_res {
        println!("Chat graph received from ship.\nWriting chat to local file...");
        let mut f = File::create(&file_path).expect("Failed to create chat export markdown file.");
        // Write markdown header into file
        writeln!(f, "# {}/{} Archive ", &args.arg_ship, &args.arg_name)
            .expect("Failed to write chat message to export markdown file.");

        // Write messages to file
        for authored_message in authored_messages {
            let markdown_message = to_markdown_string(&args, &authored_message);
            // Write the new message to the file
            writeln!(
                f,
                "_{}_ - **{}**:{}  ",
                authored_message.time_sent, authored_message.author, markdown_message
            )
            .expect("Failed to write chat message to export markdown file.");
        }

        println!(
            "Finished exporting chat to: {}\nFinished saving media files to: {}",
            file_path,
            get_content_dir(&args)
        );
    } else {
        println!("Failed to export chat. Please make sure that the `ship` & `name` are valid and are from a chat that your ship has joined.")
    }
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
