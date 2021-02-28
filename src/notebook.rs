use crate::archive::*;
use crate::Args;
use std::fs::File;
use std::io::Write;
use urbit_http_api::{comment::Comment, notebook::Note, Channel};

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
    markdown_strings.push(format!("# {}", note.title));
    markdown_strings.push(format!("##### {} - {}", note.time_sent, note.author));

    for content_lines in note.content_as_markdown() {
        let parsed_markdown = parse_link_in_markdown_string(args, &content_lines);
        markdown_strings.push(parsed_markdown);
    }

    // Add comments title if comments exist
    if note.comments.len() > 0 {
        markdown_strings.push(format!("## Comments"));
    }
    // Process the comments of the `Note`
    for comment_string in comments_to_markdown_strings(args, &note.comments) {
        markdown_strings.push(comment_string)
    }

    // Add a dividing line at the end of the note markdown
    markdown_strings.push("  ".to_string());
    markdown_strings.push("-----".to_string());

    markdown_strings
}

/// Parse markdown string for a link. If one is found, attempt to download
/// it if it is a direct file link, and update the markdown string with the new local
/// link.
pub fn parse_link_in_markdown_string(args: &Args, markdown: &str) -> String {
    let markdown = markdown.to_string();
    if let Some(bracket_start) = markdown.find("]") {
        // If no full link on a single line, then skip
        if bracket_start + 2 > markdown.len() {
            return markdown.to_string();
        }
        if let Some(bracket_end) = markdown.find(")") {
            // If no full link on a single line, then skip
            if bracket_end + 2 > markdown.len() {
                return markdown.to_string();
            }
            // Define the parts of the string
            let pre = markdown[..bracket_start].to_string();
            let url = markdown[bracket_start + 2..bracket_end].to_string();
            println!("url: {}", url);
            let post = markdown[bracket_end + 2..].to_string();

            // If url is a direct link and downloaded the file successfully
            if let Some(local_file_path) = download_file(args, &url) {
                // Return markdown with local link
                return pre.to_string() + "](" + &local_file_path + ")" + &post;
            }
        }
    }
    markdown.to_string()
}

/// Convert `Comment`s to markdown strings
pub fn comments_to_markdown_strings(args: &Args, comments: &Vec<Comment>) -> Vec<String> {
    let mut markdown_strings = vec![];
    // Process the comments of the `Note`
    for comment in comments {
        let comment_string = format!(
            "_{}_ - **{}**:{}  ",
            comment.time_sent,
            comment.author,
            message_to_markdown_string(&args, comment)
        );
        markdown_strings.push(comment_string)
    }
    markdown_strings
}
