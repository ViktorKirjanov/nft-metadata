use arloader::status::Status;
use arloader::update_statuses_stream;
use arloader::{commands::*, error::Error, status::OutputFormat, utils::TempDir, Arweave};
use futures::StreamExt;
use std::{path::PathBuf, str::FromStr};
use url::Url;

use crate::constants::{ARWEAVE_KEYPAIR_PATH, SOLANA_KEYPAIR_PATH};

#[tokio::main]
pub async fn upload_to_arweave() -> Result<Vec<Status>, Error> {
    let arweave = get_arweave().await?;

    let array = ["src/json/xxx.json"];

    let paths_iter = array.into_iter().map(PathBuf::from);

    let reward_mult = 1.0;

    let sol_keypair_path = PathBuf::from(SOLANA_KEYPAIR_PATH);

    let output_format: OutputFormat = OutputFormat::Json;
    let buffer: usize = 5;
    let temp_log_dir = TempDir::from_str("src/json/").await?;

    let log_dir = Some(temp_log_dir.0.clone());

    command_upload_with_sol(
        &arweave,
        paths_iter.clone(),
        log_dir.clone(),
        None,
        reward_mult,
        &output_format,
        buffer,
        sol_keypair_path,
    )
    .await
    .ok();

    let log_dir = PathBuf::from(log_dir.unwrap());
    let mut stream = update_statuses_stream(&arweave, paths_iter, log_dir.clone(), buffer);
    let mut counter = 0;
    let mut statuses: Vec<Status> = vec![];

    while let Some(Ok(status)) = stream.next().await {
        // if counter == 0 {
        //     println!("status header: {}", status.header_string(&output_format));
        // }
        // print!("status: {}", output_format.formatted_string(&status));
        counter += 1;
        statuses.push(status);
    }
    if counter == 0 {
        println!("The <GLOB> and <LOG_DIR> combination you provided didn't return any statuses.");
    } else {
        println!("Updated {} statuses.", counter);
    }

    Ok(statuses)
}

async fn get_arweave() -> Result<Arweave, Error> {
    let ar_keypair_path = PathBuf::from(ARWEAVE_KEYPAIR_PATH);

    let arweave = match Arweave::from_keypair_path(
        ar_keypair_path,
        Url::from_str("https://arweave.net").unwrap(),
    )
    .await
    {
        Ok(arweave) => arweave,
        Err(e) => panic!("----- arweave error: {}", e),
    };

    Ok(arweave)
}
