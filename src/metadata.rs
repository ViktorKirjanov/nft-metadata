use log::info;
use mpl_token_metadata::{
    instruction::update_metadata_accounts_v2,
    state::{DataV2, Metadata},
};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::{
    errors::DecodeError,
    models::{Attribute, UriData},
};

pub fn get_durability(uri_data: &UriData) -> Result<&Attribute, DecodeError> {
    let attribute_index = uri_data
        .attributes
        .iter()
        .position(|x| x.trait_type == "Durability");

    if let Some(index) = attribute_index {
        let attribute = &uri_data.attributes[index];
        Ok(&attribute)
    } else {
        return Err(DecodeError::NoDurabilityAttribute(
            "cannot find `Durability` attribute".to_string(),
        ));
    }
}

pub fn update_durability(mut uri_data: UriData) -> Result<UriData, DecodeError> {
    let attribute_index = uri_data
        .attributes
        .iter()
        .position(|x| x.trait_type == "Durability");

    if let Some(index) = attribute_index {
        let attribute = &mut uri_data.attributes[index];
        let value: Vec<&str> = attribute.value.split("/").collect();
        let new_durability = value[0].parse::<usize>().unwrap() - 1;
        let max_durability = value[1].parse::<usize>().unwrap();
        uri_data.attributes[index].update(format!("{}/{}", new_durability, max_durability));
        Ok(uri_data)
    } else {
        return Err(DecodeError::NoDurabilityAttribute(
            "cannot find `Durability` attribute".to_string(),
        ));
    }
}

pub fn repear_durability(mut uri_data: UriData) -> Result<UriData, DecodeError> {
    let attribute_index = uri_data
        .attributes
        .iter()
        .position(|x| x.trait_type == "Durability");

    if let Some(index) = attribute_index {
        let attribute = &mut uri_data.attributes[index];
        let value: Vec<&str> = attribute.value.split("/").collect();
        let new_durability = value[1].parse::<usize>().unwrap();
        let max_durability = value[1].parse::<usize>().unwrap();
        uri_data.attributes[index].update(format!("{}/{}", new_durability, max_durability));
        Ok(uri_data)
    } else {
        return Err(DecodeError::NoDurabilityAttribute(
            "cannot find `Durability` attribute".to_string(),
        ));
    }
}

pub fn update_uri(
    client: &RpcClient,
    keypair: &Keypair,
    new_uri: &str,
    metadata: Metadata,
    metadata_account: Pubkey,
    program_id: Pubkey,
) -> Result<(), DecodeError> {
    let update_authority = keypair.pubkey();

    let mut data = metadata.data;
    if data.uri.trim_matches(char::from(0)) != new_uri.trim_matches(char::from(0)) {
        data.uri = new_uri.to_string();

        let data_v2 = DataV2 {
            name: data.name,
            symbol: data.symbol,
            uri: data.uri,
            seller_fee_basis_points: data.seller_fee_basis_points,
            creators: data.creators,
            collection: metadata.collection,
            uses: metadata.uses,
        };

        let ix = update_metadata_accounts_v2(
            program_id,
            metadata_account,
            update_authority,
            None,
            Some(data_v2),
            None,
            None,
        );

        let recent_blockhash = client.get_latest_blockhash().unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&update_authority),
            &[keypair],
            recent_blockhash,
        );

        let sig = client.send_and_confirm_transaction(&tx).unwrap();
        info!("Tx sig: {:?}", sig);
        println!("Tx sig: {:?}", sig);
    } else {
        println!("URI is the same.");
    }

    Ok(())
}
