use lofty::read_from;
use lofty::Accessor;
use std::{fs::File, path::PathBuf};

use crate::errors::AppError;

pub struct TrackProperties {
    artist: Option<String>,
    title: Option<String>,
    album: Option<String>,
    tracknumber: Option<i32>
}

pub fn extract_metadata(path: &PathBuf) -> Result<TrackProperties, AppError> {
    File::open(path)
        .map_err(|e| AppError::CannotReadAudioMetadata { 
            file_path: path.to_string_lossy().to_string(),
            cause: format!("{:?}", e)
        })
        // TODO: I need to be able to map Result<T,E1> to Result<U,E2>
        .and_then(|mut file| match read_from(&mut file, false) {
            Ok(tags) => Ok(tags),
            Err(e) => Err(AppError::CannotReadAudioMetadata { 
                file_path: path.to_string_lossy().to_string(),
                cause: format!("{:?}", e)
            })
        })
        .map_err(|e| AppError::CannotReadAudioMetadata { 
            file_path: path.to_string_lossy().to_string(),
            cause: format!("{:?}", e)
        })
        .map(|tagged_file| {
            match tagged_file.primary_tag() {
                None => empty_track_data(),
                Some(t) => {
                    let my_tag = t.clone();
                    return TrackProperties {
                        artist: my_tag.artist().map(|s| s.to_owned()),
                        title: my_tag.title().map(|s| s.to_owned()),
                        album: my_tag.album().map(|s| s.to_owned()),
                        tracknumber: None
                    }
                }
            }
        })
}

fn empty_track_data() -> TrackProperties {
    return TrackProperties { 
        artist: None, 
        title: None, 
        album: None, 
        tracknumber: None
    }
}
