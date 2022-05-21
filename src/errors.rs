use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid configuration: {cause}")]
    InvalidConfig { cause: String },
    #[error("Cannot read audio file metadata from given file: {cause}")]
    CannotReadAudioMetadata {
        cause:String
    }
}
