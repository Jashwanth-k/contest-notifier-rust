use chrono::DateTime;
use chrono_tz::Tz;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use std::error::Error;

// struct ResponseBody<'a> {
//     duration : Option<u32>,
//     end : Option<&'a str>,
//     event : Option<&'a str>,
//     host : Option<&'a str>,
//     href : Option<&'a str>,
//     id : Option<u64>,
//     n_problems : Option<u8>,
//     n_statistics : Option<u8>,
//     resource: Option<&'a str>,
//     resource_id : Option<u8>,
//     start : Option<&'a str>
// }

#[derive(serde::Deserialize, Debug)]
pub struct ResponseBody {
    meta: Option<Meta>,
    objects: Option<Vec<ResponseObject>>,
    error: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Meta {
    estimated_count: Option<u64>,
    limit: Option<u32>,
    next: Option<String>,
    offset: Option<u32>,
    previous: Option<u32>,
    total_count: Option<u64>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ResponseObject {
    pub duration: Option<u32>,
    pub end: Option<String>,
    pub event: Option<String>,
    pub host: Option<String>,
    pub href: Option<String>,
    pub id: Option<u64>,
    pub n_problems: Option<u8>,
    pub n_statistics: Option<u32>,
    pub parsed_at: Option<String>,
    pub problems: Option<String>,
    pub resource: Option<String>,
    pub resource_id: Option<u8>,
    pub start: Option<String>,
}

pub mod main {
    use super::*;
    static CLIST_URL: &str =
        "https://clist.by/api/v4/contest/?start__gt=${startDate}&end__lt=${endDate}&order_by=start";

    // #[tokio::main]
    pub async fn get_contest_data<'a>(
        start_date: DateTime<Tz>,
        end_date: DateTime<Tz>,
    ) -> Result<Vec<ResponseObject>, Box<dyn Error>> {
        let authorization: &str = &env::var("CLIST_API_KEY")?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.append("content_type", HeaderValue::from_static("application/json"));
        headers.append("authorization", HeaderValue::from_str(&authorization)?);

        let apiurl: String = String::from(CLIST_URL)
            .replace(
                "${startDate}",
                &start_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
            )
            .replace(
                "${endDate}",
                &end_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
            );
        let headers2 = headers.clone();

        let response: reqwest::Response = client.get(&apiurl).headers(headers).send().await?;
        let resp_body: ResponseBody = response.json::<ResponseBody>().await?;
        if resp_body.error.is_some() {
            return Err(resp_body.error.unwrap().into());
        }
        if resp_body.objects.is_none() {
            return Err("object not found".into());
        }
        return Ok(resp_body.objects.unwrap());
    }
}
