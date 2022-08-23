// use solana_client::client_error::ClientErrorKind;
use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum ErrorCode {
//     #[error("The provided valuee should be more than 0.")]
//     ValueTooSmall,

//     #[error("The provided value should not be greater than 50.")]
//     ValueTooBig,
// }

#[derive(Error, Debug)]
pub enum DecodeError {
    // #[error("no account data found")]
    // MissingAccount(String),
    // #[error("failed to get accoun?t data")]
    // ClientError(ClientErrorKind),
    #[error("network request failed after three attempts: ensure you used a valid address and check the state of the Solana cluster")]
    NetworkError(String),

    #[error("failed to parse string into Pubkey")]
    PubkeyParseFailed(String),

    #[error("failed to decode metadata")]
    DecodeMetadataFailed(String),

    #[error("failed to get an request")]
    RequestFailed(String),

    #[error("response didn't match the shape we expected.")]
    ShapeFailed(String),

    #[error("cannot find `Durability` attribute.")]
    NoDurabilityAttribute(String),
}
