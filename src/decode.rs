use crate::errors::DecodeError;
use borsh::BorshDeserialize;
use mpl_token_metadata::state::Metadata;
use retry::{delay::Exponential, retry};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use std::str::FromStr;

pub fn str_to_pubkey(address: &str) -> Result<Pubkey, DecodeError> {
    let pubkey = match Pubkey::from_str(address) {
        Ok(pubkey) => pubkey,
        Err(err) => {
            return Err(DecodeError::PubkeyParseFailed(err.to_string().into()));
        }
    };
    Ok(pubkey)
}

pub fn get_metadata_pda(token_pubkey: Pubkey, metaplex_pubkey: Pubkey) -> Pubkey {
    let seeds = &[
        "metadata".as_bytes(),
        metaplex_pubkey.as_ref(),
        token_pubkey.as_ref(),
    ];

    let (pda, _) = Pubkey::find_program_address(seeds, &metaplex_pubkey);
    pda
}

pub fn get_metadata(client: &RpcClient, token_pda: Pubkey) -> Result<Metadata, DecodeError> {
    let account_data = match retry(
        Exponential::from_millis_with_factor(250, 2.0).take(3),
        || client.get_account_data(&token_pda),
    ) {
        Ok(data) => data,
        Err(err) => {
            return Err(DecodeError::NetworkError(err.to_string()));
        }
    };

    let metadata: Metadata = match Metadata::deserialize(&mut account_data.as_slice()) {
        Ok(m) => m,
        Err(err) => return Err(DecodeError::DecodeMetadataFailed(err.to_string())),
    };

    Ok(metadata)
}

pub fn parse_keypair(keypair_path: String) -> Keypair {
    read_keypair_file(&keypair_path).expect("Failed to read keypair file.")
}

#[cfg(test)]
mod tests {

    use crate::constants::METAPLEX_PROGRAM_ID;

    use super::*;

    #[test]
    fn test_str_to_pubkey_ok() {
        let metaplex_pubkey = str_to_pubkey(METAPLEX_PROGRAM_ID).unwrap();
        assert_eq!(metaplex_pubkey.to_string(), METAPLEX_PROGRAM_ID);
    }

    #[test]
    #[should_panic(expected = "PubkeyParseFailed")]
    fn test_str_to_pubkey_error() {
        str_to_pubkey("0").unwrap();
    }

    #[test]
    fn test_get_metadata_pda_ok() {
        let metaplex_pubkey = str_to_pubkey(METAPLEX_PROGRAM_ID).unwrap();
        let item_pubkey = str_to_pubkey("EZYnxHFZgWGZHUrYi6BCTk8n4bmZ9MFgceLGvQsWNiW6").unwrap();
        let item_pda = get_metadata_pda(item_pubkey, metaplex_pubkey);
        assert_eq!(
            item_pda.to_string(),
            "HdfkpeatiWLCEe3kBdUgpgwSVWhij6sgTquNnjbsWVbq"
        );
    }
}
