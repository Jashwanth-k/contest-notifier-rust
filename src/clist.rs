use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

pub mod clist {
    use std::error::Error;

    use super::*;
    static CLIST_URL: &str =
        "https://clist.by/api/v4/contest/?start__gt=${startDate}&end__lt=${endDate}&order_by=start";

    pub async fn get_contest_data<'a>(
        start_date: &str,
        end_date: &str,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let authorization: String = match env::var("CLIENT_API_KEY") {
            Ok(val) => val,
            Err(_e) => "".to_string(),
        };

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.append("content_type", HeaderValue::from_static("application/json"));
        headers.append("authorization", HeaderValue::from_str(&authorization)?);

        let apiurl: String = String::from(CLIST_URL)
            .replace("startDate", &start_date)
            .replace("endDate", &end_date);

        let response = client.get(apiurl).headers(headers).send().await;
        return Ok(response?);
    }
}
