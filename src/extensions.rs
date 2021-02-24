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
