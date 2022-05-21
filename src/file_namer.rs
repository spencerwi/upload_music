use std::path::PathBuf;

use crate::audioutils::TrackMetadata;

pub fn get_filename(track_metadata : &TrackMetadata, filename_pattern: &String, file_extension: Option<&str>) -> PathBuf {
    let replaced_str : String = filename_pattern.replace("{{ARTIST}}", &track_metadata.artist.as_ref().unwrap_or(&String::from("")))
            .replace("{{ALBUM}}", &track_metadata.album.as_ref().unwrap_or(&String::from("")))
            .replace("{{TITLE}}", &track_metadata.title.as_ref().unwrap_or(&String::from("")))
            .replace("{{TRACKNUMBER}}", &track_metadata.tracknumber.map(|n| n.to_string()).unwrap_or(String::from("")));
    match file_extension {
        Some(ext) => return PathBuf::from(format!("{}.{}", replaced_str, ext)),
        None => return PathBuf::from(replaced_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Component;
    use std::ffi::OsStr;

    fn dummy_metadata() -> TrackMetadata {
        return TrackMetadata { 
            artist: Some("Artist Name".to_string()), 
            title: Some("Track Title".to_string()),
            album: Some("Album Name".to_string()), 
            tracknumber: Some(1)
        };
    }

    #[test]
    fn test_get_filename_with_literal_pattern() {
        let metadata = dummy_metadata();
        let pattern = String::from("a literal pattern");
        assert_eq!(PathBuf::from(format!("{}.{}", pattern, "mp3")), get_filename(&metadata, &pattern, Some("mp3")));
    }

    #[test]
    fn test_get_filename_with_directory_in_literal_pattern() {
        let metadata = dummy_metadata();
        let pattern = String::from("one/two/three");

        let result = get_filename(&metadata, &pattern, Some("mp3"));
        let expected_result = Vec::from([
            Component::Normal(OsStr::new("one")),
            Component::Normal(OsStr::new("two")),
            Component::Normal(OsStr::new("three.mp3")),
        ]);
        let folders : Vec<_> = result.components().collect();
        assert_eq!(expected_result, folders);
    }

    #[test]
    fn test_get_filename_with_placeholders() {
        let metadata = dummy_metadata();
        let pattern = String::from("{{ARTIST}}_{{ALBUM}}_{{TRACKNUMBER}}_{{TITLE}}");
        assert_eq!(PathBuf::from("Artist Name_Album Name_1_Track Title.mp3"), get_filename(&metadata, &pattern, Some("mp3")));
    }

    #[test]
    fn test_get_filename_with_directory_with_placeholders() {
        let metadata = dummy_metadata();
        let pattern = String::from("{{ARTIST}}/{{ALBUM}}/{{TRACKNUMBER}} - {{TITLE}}");

        let result = get_filename(&metadata, &pattern, Some("mp3"));
        let expected_result = Vec::from([
            Component::Normal(OsStr::new("Artist Name")),
            Component::Normal(OsStr::new("Album Name")),
            Component::Normal(OsStr::new("1 - Track Title.mp3")),
        ]);
        let folders : Vec<_> = result.components().collect();
        assert_eq!(expected_result, folders);
    }
}
