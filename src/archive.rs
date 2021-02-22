use crate::Args;
use std::{fs, fs::File, path::Path};

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

    // Create and save the file
    let mut file = File::create(&path).ok()?;
    match std::io::copy(&mut image_file, &mut file) {
        Err(_) => panic!("couldn't write to {}", &path.to_string_lossy()),
        Ok(_) => Some(path_string),
    }
}

/// Creates the archived content directory
pub fn create_content_dir(args: &Args) {
    let content_dir = get_content_dir(args);
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
