use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Decoding JABCode failed")]
    Jab,
    #[error("JABCode Partially Decoded")]
    PartialDecode,
    #[error("JABCode Detected but Decoding failed")]
    DecodeFailed
}
