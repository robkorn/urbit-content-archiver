use crate::archive::*;
use crate::Args;
use std::fs::File;
use std::io::Write;
use urbit_http_api::Channel;

/// Exports the chat resource provided via arguments
pub fn export_chat(args: Args, channel: &mut Channel) {
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
            let markdown_message = message_to_markdown_string(&args, &authored_message);
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
