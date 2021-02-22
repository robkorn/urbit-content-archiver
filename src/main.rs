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
        urbit-content-archiver chat <chat-ship> <chat-name> [--config=<file_path> --output=<folder_path>]
Options:
      --config=<file_path>  Specify a custom path to a YAML ship config file.
      --output=<folder_path>  Specify a custom path where the output file will be saved.

"#;

#[derive(Debug, Deserialize)]
struct Args {
    cmd_chat: bool,
    arg_chat_ship: String,
    arg_chat_name: String,
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
    let file_name = format!("{}-{}.txt", &args.arg_chat_ship[1..], &args.arg_chat_name);
    let file_path = match args.flag_output.is_empty() {
        true => file_name,
        false => format!("{}/{}", args.flag_output, file_name),
    };

    println!(
        "Requesting {}/{} chat graph from your ship...",
        &args.arg_chat_ship, &args.arg_chat_name
    );

    // Acquire the chat log from the ship
    let chat_log_res = channel
        .chat()
        .export_chat_log(&args.arg_chat_ship, &args.arg_chat_name);

    // Save to file if acquiring and processing the chat log was successful
    if let Ok(chat_log) = chat_log_res {
        println!("Chat graph received from ship.\nWriting chat to local file...");

        let mut f = File::create(&file_path).expect("Failed to create chat export text file.");
        // Write messages to file
        for message in chat_log {
            writeln!(f, "{}", message).expect("Failed to write chat message to export text file.");
        }

        println!("Finished saving chat to: {}", file_path);
    } else {
        println!("Failed to export chat. Please make sure that the `chat_ship` & `chat_name` are valid and are from a chat that your ship has joined.")
    }
}

/// Basic setup of generating a config file and getting a `ShipInterface` from local config.
fn basic_setup() -> (ShipInterface, Args) {
    // Read command line arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

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
