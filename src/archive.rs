use crate::Args;
use json::{object, JsonValue};
use std::{fs, fs::File, path::Path};
use urbit_http_api::chat::{AuthoredMessage, Message};

/// Downloads a file to the archived content directory.
/// Returns the path as a `String` on success.
pub fn download_file(args: &Args, url: &str) -> Option<String> {
    let client = reqwest::blocking::Client::new();

    // Acquire file name and path
    let split_url: Vec<&str> = url.split("/").collect();
    let file_name = split_url[split_url.len() - 1];
    let download_path_string = get_content_dir(args) + "/" + file_name;
    let path = Path::new(&download_path_string);

    // If the file does not exist already, download it
    if !path.exists() {
        println!("Downloading {}...", file_name);
        let mut downloaded_file = client.get(url).send().ok()?;
        // Create and save the file
        let mut file = File::create(&path).ok()?;
        let _res = std::io::copy(&mut downloaded_file, &mut file).ok()?;
    } else {
        println!("Already downloaded {}, skipping.", file_name);
    }
    // Return the inner path to the file in the `archived-content` folder
    Some(format!("archived-content/{}", file_name))
}

/// Given a `&str` website URL, checks if it is a media file which can be downloaded
pub fn is_media_file_url(url: &str) -> bool {
    let extensions = vec![
        "png", "jpg", "jpeg", "gif", "gifv", "mov", "qt", "svg", "mp4", "m4v", "mpv", "mpg",
        "mpeg", "mp2", "3gp", "3gp2", "mpe", "webm", "mkv", "mov", "wmv", "wav", "flv", "avi",
        "ogv", "flac", "ape", "mp3", "wav", "m4a", "opus", "aac", "m4b", "ogg", "oga", "raw",
    ];
    matches_file_extensions(url, extensions)
}

/// Given a `&str` website URL, checks if it is a downloadable file
pub fn is_downloadable_file_url(url: &str) -> bool {
    let extensions = vec![
        "pdf", "md", "txt", "epub", "mobi", "djvu", "doc", "docx", "fb2", "azw", "azw3", "kf8",
        "kfx", "prc", "cbr", "torrent", "iso", "tar", "gz", "bz2", "lz", "lz4", "lzma", "lzo",
        "bz", "bz2", "Z", "tbz2", "tlz", "rz", "xz", "zst", "txz", "zip", "7z", "ace", "apk",
        "arc", "ark", "dmg", "jar", "rar",
    ];
    matches_file_extensions(url, extensions)
}

/// Checks if given url is a file which matches one from the list of extensions
fn matches_file_extensions(url: &str, extensions: Vec<&str>) -> bool {
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
pub fn message_to_markdown_string(args: &Args, authored_message: &AuthoredMessage) -> String {
    let mut new_content_list = vec![];
    for json in &authored_message.contents.content_list {
        // If the json content is a URL
        if !json["url"].is_empty() {
            // Get the URL and convert it into a markdown string
            let url = format!("{}", json["url"]);
            new_content_list.push(download_and_convert_to_markdown(&args, &url));
        } else {
            new_content_list.push(json.clone())
        }
    }
    // The new `Message` that has had any media links downloaded & processed
    let new_message = Message::from_json(new_content_list);
    new_message.to_formatted_string()
}

/// Downloads directly linked content at the provided URL and converts local link to markdown
/// and embeds it within a `NodeContent` schemed `JsonValue`
pub fn download_and_convert_to_markdown(args: &Args, url: &str) -> JsonValue {
    let split_url: Vec<&str> = url.split("/").collect();
    let file_name = split_url[split_url.len() - 1];
    // If the URL is a media file
    if is_media_file_url(&url) {
        // Download the media file locally and add image markdown
        if let Some(file_path) = download_file(&args, &url) {
            let markdown_json = object! {
                "text": format!("![{}]({})", file_name, &file_path)
            };
            return markdown_json;
        }
        return object! {};
    }
    // Download file locally and add link markdown
    else if is_downloadable_file_url(&url) {
        // download the media file locally and add location to text
        if let Some(file_path) = download_file(&args, &url) {
            return object! {
                "text": format!("[{}]({})", file_name, &file_path)
            };
        }
        return object! {};
    }
    // If it's not a media file, it's just a normal url which needs to be linked
    else {
        return object! {
            "text": format!("[{}]({})", file_name, url)
        };
    }
}
