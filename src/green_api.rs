use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use serde_json::json;
use std::env;
use std::error::Error;

#[derive(Serialize, Debug)]
struct RequestBody<'a> {
    chatId: &'a str,
    message: &'a String,
}

pub mod green_api {
    use super::*;
    static API_URL: &str =
        "https://${apiHost}/waInstance${idInstance}/sendMessage/${tokenInstance}";
    pub async fn sendMessage(message: &String) -> Result<reqwest::Response, Box<dyn Error>> {
        let api_host: &str = &env::var("GREEN_API_HOST")?;
        let id_instance: &str = &env::var("GREEN_INSTANCE_ID")?;
        let token_instance: &str = &env::var("GREEN_TOKEN_INSTANCE")?;
        let mobile: &str = &env::var("MOBILE")?;

        let mut headers = HeaderMap::new();
        headers.append("content-type", HeaderValue::from_static("application/json"));
        let body: RequestBody = RequestBody {
            chatId: &"${mobile}@c.us".replace("${mobile}", mobile),
            message,
        };
        let json_body = json!(body);

        let api_url = String::from(API_URL)
            .replace("${apiHost}", api_host)
            .replace("${idInstance}", id_instance)
            .replace("${tokenInstance}", token_instance);

        let client = reqwest::Client::new();
        let response = client
            .post(api_url)
            .headers(headers)
            .body(json_body.to_string())
            .send()
            .await;
        return Ok(response?);
    }
}
