
use std::env;
use lazy_static::lazy_static;
use reqwest::*;
use std::collections::HashMap;
use reqwest::header::HeaderMap;

// struct Headers<'a> {
//     content_type : &'a str,
//     authorization : Option<String>,
// }

pub mod clist {


    use reqwest::header::HeaderValue;

    use super::*;
    static CLIST_URL: &str = "https://clist.by/api/v4/contest/?start__gt=${startDate}&end__lt=${endDate}&order_by=start";

    pub async fn get_contest_data<'a> (start_date: &str, end_date : &str) {
        let authorization: Option<String> = match env::var("CLIENT_API_KEY") {
            Ok(val) => Some(val),
            Err(_e) => Some(""),
        };

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.append("content_type", HeaderValue::from_static(CLIST_URL));
        headers.append("authorization", HeaderValue::from_static(authorization));
        // headers.insert("authorization", authorization);
        
        let mut apiurl: String = String::from(CLIST_URL);
        apiurl = apiurl.replace("startDate", &start_date);
        apiurl = apiurl.replace("endDate", &end_date);

        let response = client.get(apiurl)
            .headers(headers)
            .send()
            .await()
    }
}
