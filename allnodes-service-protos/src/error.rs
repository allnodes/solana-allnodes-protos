use {thiserror::Error, tonic::Status};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Conversion error: {0}")]
    ConversionError(#[from] ConversionError),
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Integer conversion error: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Pubkey deserialization error from bytes: {0:?}")]
    PubkeyDeserialization(Vec<u8>),
}

impl From<ConversionError> for Status {
    fn from(from: ConversionError) -> Self {
        Status::internal(from.to_string())
    }
}
