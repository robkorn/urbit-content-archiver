use crate::archive::*;
use crate::Args;
use std::fs::File;
use std::io::Write;
use urbit_http_api::{chat::Message, notebook::Note, Channel};

/// Exports the notebook resource provided via arguments
pub fn export_notebook(args: Args, channel: &mut Channel) {
    // Set the path where the file will be saved
    let file_name = format!("{}-{}.md", &args.arg_ship[1..], &args.arg_name);
    let file_path = format!("{}/{}", get_root_dir(&args), file_name);

    println!(
        "Requesting {}/{} chat graph from your ship...",
        &args.arg_ship, &args.arg_name
    );

    // Acquire the authored messages from the ship
    let notes_res = channel
        .notebook()
        .export_notebook(&args.arg_ship, &args.arg_name);

    // Parse the authored message, save files, and save chat messages.
    if let Ok(notes) = notes_res {
        println!("Chat graph received from ship.\nWriting chat to local file...");
        let mut f = File::create(&file_path).expect("Failed to create chat export markdown file.");
        // Write markdown header into file
        writeln!(f, "# {}/{} Archive ", &args.arg_ship, &args.arg_name)
            .expect("Failed to write to export markdown file.");

        // Write notes to file
        for note in notes {
            for markdown_line in note_to_markdown_strings(&args, &note) {
                writeln!(f, "{}", markdown_line).expect("Failed to write to export markdown file.");
            }
        }

        println!(
            "Finished exporting notebook to: {}\nFinished saving media files to: {}",
            file_path,
            get_content_dir(&args)
        );
    } else {
        println!("Failed to export notebook. Please make sure that the `ship` & `name` are valid and are from a notebook that your ship has joined.")
    }
}

/// Convert a `Note` into a set of prepared markdown `String`s
/// with the content files downloaded
pub fn note_to_markdown_strings(args: &Args, note: &Note) -> Vec<String> {
    let mut markdown_strings = vec![];
    markdown_strings.push(format!("## {}", note.title));
    markdown_strings.push(format!("##### {} - {}", note.time_sent, note.author));

    // Process the content of the `Note`
    let mut new_content_list = vec![];
    for json in &note.content.content_list {
        // If the json content is a URL
        if !json["url"].is_empty() {
            // Get the URL and convert it into a markdown string
            let url = format!("{}", json["url"]);
            new_content_list.push(download_and_convert_to_markdown(&args, &url));
        } else {
            new_content_list.push(json.clone())
        }
    }
    // Use the `Message` .to_formatted_string() method to process the note
    let processed_content_string = Message::from_json(new_content_list).to_formatted_string();
    markdown_strings.push(processed_content_string);

    // Process the comments of the `Note`
    // let mut new_content_list = vec![];
    for json in &note.comments {}

    markdown_strings
}
