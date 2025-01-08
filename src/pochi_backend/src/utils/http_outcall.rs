use ic_cdk::api::call::CallResult;
use ic_cdk::api::http::{self, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HttpRequestData {
    url: String,
    body: String,
    headers: Vec<(String, String)>,
}

#[update]
async fn http_outcall(request_data: HttpRequestData) -> CallResult<Vec<u8>> {
    let client = reqwest::Client::new();
    
    let request = Request::builder()
        .method("POST")
        .uri(&request_data.url)
        .body(request_data.body)
        .headers(request_data.headers)
        .send()
        .await?;

    let response = request.text().await?;
    Ok(response.as_bytes().to_vec())
}
