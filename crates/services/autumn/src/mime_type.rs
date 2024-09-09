use tempfile::NamedTempFile;

/// Determine the mime type of the given temporary file and filename
pub fn determine_mime_type(f: &mut NamedTempFile, buf: &Vec<u8>, file_name: &str) -> &'static str {
    // Use magic signatures to determine mime type
    let kind = infer::get_from_path(f.path()).expect("file read successfully");
    let mime_type = if let Some(kind) = kind {
        kind.mime_type()
    } else {
        "application/octet-stream"
    };

    // Map any known conflicts where appropriate
    let mime_type = if mime_type == "application/zip" && file_name.to_lowercase().ends_with(".apk")
    {
        "application/vnd.android.package-archive"
    } else {
        mime_type
    };

    // See if the file is actually just plain Unicode/ASCII text
    if mime_type == "application/octet-stream" && simdutf8::basic::from_utf8(buf).is_ok() {
        return "plain/text";
    }

    mime_type
}