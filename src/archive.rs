use crate::Args;
use json::object;
use std::{fs, fs::File, path::Path};
use urbit_http_api::chat::{AuthoredMessage, Message};

/// Downloads a file to the archived content directory.
/// Returns the path as a `String` on success.
pub fn download_file(args: &Args, url: &str) -> Option<String> {
    let client = reqwest::blocking::Client::new();
    let mut image_file = client.get(url).send().ok()?;

    // Acquire file name and path
    let split_url: Vec<&str> = url.split("/").collect();
    let image_name = split_url[split_url.len() - 1];
    let path_string = get_content_dir(args) + "/" + image_name;
    let path = Path::new(&path_string);

    println!("Downloading {}", image_name);

    // Create and save the file
    let mut file = File::create(&path).ok()?;
    match std::io::copy(&mut image_file, &mut file) {
        Err(_) => panic!("couldn't write to {}", &path.to_string_lossy()),
        Ok(_) => Some(path_string),
    }
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
    let content_dir = get_content_dir(args);
    // Create base directory
    let _res = fs::create_dir("archived-content");
    // Create sub directory
    let _res = fs::create_dir(content_dir);
}

/// Acquires the current content directory based on the argument flags
pub fn get_content_dir(args: &Args) -> String {
    let mut path_string =
        "archived-content".to_string() + "/" + &args.arg_ship + "-" + &args.arg_name;
    if !args.flag_output.is_empty() {
        path_string = args.flag_output.to_string() + "/" + &path_string;
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
