use crate::{errors::DecodeError, models::UriData};

#[tokio::main]
pub async fn get_uri_data(uri: &str) -> Result<UriData, DecodeError> {
    let client = reqwest::Client::new();

    let response = client.get(uri).send().await;

    let response_ok = match response {
        Ok(response) => response,
        Err(err) => return Err(DecodeError::RequestFailed(err.to_string())),
    };

    match response_ok.status() {
        reqwest::StatusCode::OK => reqwest::StatusCode::OK,
        _ => {
            return Err(DecodeError::RequestFailed("xxx".to_string()));
        }
    };

    let parsed = match response_ok.json::<UriData>().await {
        Ok(parsed) => parsed,
        // Err(e) => panic!("Hm, the response didn't match the shape we expected."),
        Err(err) => return Err(DecodeError::ShapeFailed(err.to_string())),
    };

    Ok(parsed)
}
