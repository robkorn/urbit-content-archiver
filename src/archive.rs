use crate::Args;
use json::object;
use std::{fs, fs::File, path::Path};
use urbit_http_api::chat::{AuthoredMessage, Message};

/// Downloads a file to the archived content directory.
/// Returns the path as a `String` on success.
pub fn download_file(args: &Args, url: &str) -> Option<String> {
    let client = reqwest::blocking::Client::new();
    let mut media_file = client.get(url).send().ok()?;

    // Acquire file name and path
    let split_url: Vec<&str> = url.split("/").collect();
    let media_name = split_url[split_url.len() - 1];
    let download_path_string = get_content_dir(args) + "/" + media_name;
    let path = Path::new(&download_path_string);

    // If the file does not exist already, download it
    if !path.exists() {
        println!("Downloading {}...", media_name);
        // Create and save the file
        let mut file = File::create(&path).ok()?;
        let _res = std::io::copy(&mut media_file, &mut file).ok()?;
    } else {
        println!("Already downloaded {}, skipping.", media_name);
    }
    // Return the inner path to the file in the `archived-content` folder
    Some(format!("archived-content/{}", media_name))
}

/// Given a `&str` website URL, checks if it is a media file which can be downloaded
pub fn is_media_file_url(url: &str) -> bool {
    let extensions = vec![
        "png", "jpg", "jpeg", "gif", "svg", "mp4", "m4v", "webm", "mkv", "mov", "wmv", "wav",
        "flv", "avi",
    ];
    let split_url: Vec<&str> = url.split(".").collect();

    for ext in extensions {
        if ext == split_url[split_url.len() - 1] {
            return true;
        }
    }
    false
}

/// Creates the archived content directory
pub fn create_content_dir(args: &Args) {
    // Create root archive directory
    let _res = fs::create_dir(get_root_dir(args));
    // Create content sub-directory
    let _res = fs::create_dir(get_content_dir(args));
}

/// Acquires the current content directory based on the argument flags
pub fn get_content_dir(args: &Args) -> String {
    get_root_dir(args) + "/" + "archived-content"
}

/// Get the root directory for the archive
pub fn get_root_dir(args: &Args) -> String {
    let mut path_string = args.arg_ship[1..].to_string() + "-" + &args.arg_name;
    if !args.flag_output.is_empty() {
        path_string = args.flag_output.to_string();
    }
    path_string
}

/// Convert an `AuthoredMessage` into a single markdown `String`
/// with the media files
pub fn to_markdown_string(args: &Args, authored_message: &AuthoredMessage) -> String {
    let mut new_content_list = vec![];
    for json in &authored_message.message.content_list {
        // If the json content is a URL
        if !json["url"].is_empty() {
            // If the URL is a media file
            let url = format!("{}", json["url"]);
            if is_media_file_url(&url) {
                // download the media file locally and add location to text
                if let Some(file_path) = download_file(&args, &url) {
                    let markdown_json = object! {
                        "text": format!("![]({})", &file_path)
                    };
                    new_content_list.push(markdown_json);
                }
            }
            // If it's not a media file, it's just a normal url which needs to be linked
            else {
                let markdown_json = object! {
                    "text": format!("[{}]({})", url, url)
                };
                new_content_list.push(markdown_json);
            }
        } else {
            new_content_list.push(json.clone())
        }
    }
    // The new `Message` that has had any media links downloaded & processed
    let new_message = Message::from_json(new_content_list);
    new_message.to_formatted_string()
}
