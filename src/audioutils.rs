extern crate tree_magic;

use lofty::Probe;
use lofty::Accessor;

use crate::errors::AppError;

pub struct TrackMetadata {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub tracknumber: Option<i32>
}

pub fn is_supported_audiofile(contents : &Vec<u8>) -> bool {
    match tree_magic::from_u8(&contents).as_str() {
        "audio/x-aiff" => true,
        "audio/x-ape" => true,
        "audio/flac" => true,
        "audio/mpeg" => true, // mp3 or mp4 audio
        "audio/mp3" => true,
        "audio/ogg" => true,
        "audio/opus" => true,
        "audio/speex" => true,
        "audio/wav" => true,
        "audio/wave" => true,
        "audio/x-wav" => true,
        "audio/x-pn-wav" => true,
        _ => false
    }
}

pub fn extract_metadata<R: std::io::Read + std::io::Seek>(source: &mut R) -> Result<TrackMetadata, AppError> {
    return Probe::new(source).read(true)
        .map_err(|e| AppError::CannotReadAudioMetadata { 
            cause: format!("{:?}", e)
        })
        .map(|tagged_file| {
            match tagged_file.primary_tag() {
                None => empty_track_data(),
                Some(t) => {
                    let my_tag = t.clone();
                    return TrackMetadata {
                        artist: my_tag.artist().map(|s| s.to_owned()),
                        title: my_tag.title().map(|s| s.to_owned()),
                        album: my_tag.album().map(|s| s.to_owned()),
                        tracknumber: None // TODO: how to read track number?
                    }
                }
            }
        })
}

fn empty_track_data() -> TrackMetadata {
    return TrackMetadata { 
        artist: None, 
        title: None, 
        album: None, 
        tracknumber: None
    }
}
