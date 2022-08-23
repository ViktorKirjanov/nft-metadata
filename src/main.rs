mod constants;
mod decode;
mod errors;
mod metadata;
mod models;
mod request;
mod upload;

use std::{env, io};

use crate::{
    constants::{METAPLEX_PROGRAM_ID, SOLANA_KEYPAIR_PATH},
    decode::parse_keypair,
    upload::upload_to_arweave,
};
use colored::Colorize;
use mpl_token_metadata::state::{Data, Metadata};
use solana_client::rpc_client::RpcClient;

fn main() {
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "full");

    let nfts = [
        "GvjmM15fi8yvsHxRMVPayAuwF72pZRwKie3NzHyXXPhH",
        "934V3uTFXnToCNkJGnUinFt3AUvW3TtiV9J3dMxbQB9N",
        "EZYnxHFZgWGZHUrYi6BCTk8n4bmZ9MFgceLGvQsWNiW6",
    ];

    loop {
        println!("\n\n=========================");
        println!("Available NFTs: (1,2,3)");
        for index in 0..nfts.len() {
            println!("index: {}: {}", index + 1, nfts[index]);
        }
        println!("=========================");
        println!("{}", "Please, select index of NFT:".yellow());

        let mut selected_index = String::new();
        io::stdin()
            .read_line(&mut selected_index)
            .expect("Failed to read the line");

        let selected_index: usize = match selected_index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if selected_index <= 0 || selected_index > nfts.len() {
            println!("{}", "Wrong index. Try again".red());
            continue;
        }
        let selected_nft_address = nfts[selected_index - 1];
        println!("Selected NFT: {}", selected_nft_address);

        let metaplex_pubkey = match decode::str_to_pubkey(METAPLEX_PROGRAM_ID) {
            Ok(pubkey) => pubkey,
            Err(err) => match err {
                err => {
                    log::error!(":{}", err);
                    continue;
                }
            },
        };

        let token_pubkey = match decode::str_to_pubkey(selected_nft_address) {
            Ok(pubkey) => pubkey,
            Err(err) => match err {
                err => {
                    log::error!(":{}", err);
                    continue;
                }
            },
        };

        let token_pda = decode::get_metadata_pda(token_pubkey, metaplex_pubkey);

        let url = "https://api.devnet.solana.com".to_string();
        let client = RpcClient::new(url);

        let metadata: Metadata = match decode::get_metadata(&client, token_pda) {
            Ok(m) => m,
            Err(err) => match err {
                err => {
                    log::error!(
                        "Failed to decode metadata for mint account: {}, error: {}",
                        token_pda,
                        err
                    );
                    continue;
                }
            },
        };

        // print!("metaplex pubkey: {}\n\n", metaplex_pubkey);
        // print!("token pubkey: {}\n\n", token_pubkey);
        // print!("item pda: {}\n\n", token_pda);
        // print!("metadata: {:?}\n\n", metadata);

        // TODO: WTF??? let data: Data = metadata.data; dosnt work
        let data: Data = (match decode::get_metadata(&client, token_pda) {
            Ok(m) => m,
            Err(err) => match err {
                err => {
                    log::error!(
                        "Failed to decode metadata for mint account: {}, error: {}",
                        token_pda,
                        err
                    );
                    continue;
                }
            },
        })
        .data;
        // print!("data: {:?}\n\n", data);

        let uri = data.uri.trim_matches(char::from(0));
        println!("uri: {:?}", uri);
        println!("Fetching data ...");
        let uri_data = match request::get_uri_data(uri) {
            Ok(uri_data) => uri_data,
            Err(err) => match err {
                err => {
                    log::error!("Failed to get uri data, error: {}", err);
                    continue;
                }
            },
        };
        // println!("NFT attributes: {:#?}", uri_data.attributes);

        let attribute = match metadata::get_durability(&uri_data) {
            Ok(a) => a,
            Err(err) => match err {
                err => {
                    log::error!(
                        "Failed to find durability attribute: {}, error: {}",
                        token_pda,
                        err
                    );
                    continue;
                }
            },
        };
        println!("NFT durability: {}", attribute.value.green());

        loop {
            println!("{}", "Please, select action:".yellow());
            println!("1. Use");
            println!("2. Repear");

            let mut selected_index = String::new();
            io::stdin()
                .read_line(&mut selected_index)
                .expect("Failed to read the line");

            let selected_index: usize = match selected_index.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("Uploading new off-chain json file to Arweave ...");
            match selected_index {
                1 => {
                    let new_uri_data = match metadata::update_durability(uri_data) {
                        Ok(uri_data) => uri_data,
                        Err(err) => match err {
                            err => {
                                log::error!("Failed to get uri data, error: {}", err,);
                                return;
                            }
                        },
                    };

                    // Save the JSON structure into the file.
                    // TODO: unwrap + errors
                    let serialized = serde_json::to_string_pretty(&new_uri_data).unwrap();
                    std::fs::write("src/json/xxx.json", serialized).unwrap();
                }
                2 => {
                    let new_uri_data = match metadata::repear_durability(uri_data) {
                        Ok(uri_data) => uri_data,
                        Err(err) => match err {
                            err => {
                                log::error!("Failed to get uri data, error: {}", err,);
                                return;
                            }
                        },
                    };

                    // Save the JSON structure into the file.
                    // TODO: unwrap + errors
                    let serialized = serde_json::to_string_pretty(&new_uri_data).unwrap();
                    std::fs::write("src/json/xxx.json", serialized).unwrap();
                }
                _ => continue,
            }

            let statuses = upload_to_arweave();
            let statuses = match statuses {
                Ok(s) => s,
                Err(_) => {
                    return;
                }
            };

            let transaction_id = &statuses[0].id.to_string();
            let new_uri = &format!("https://arweave.net/{}", transaction_id);
            println!("new off-chain uri {}", new_uri.green());
            println!("Updating metadata ...");

            let keypair = parse_keypair(String::from(SOLANA_KEYPAIR_PATH));

            metadata::update_uri(
                &client,
                &keypair,
                new_uri,
                metadata,
                token_pda,
                metaplex_pubkey,
            )
            .expect("Couldn’t update URI");

            break;
        }
    }
}
