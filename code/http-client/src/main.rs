use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;


async fn get() -> Result<HashMap<String, String>, reqwest::Error>{
    let res = reqwest::get("http://127.0.0.1:9090/json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(res)
}

async fn post() -> Result<HashMap<String, Value>, reqwest::Error>{
    // new client
    let client = reqwest::Client::new();

    // header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut data = HashMap::new();
    data.insert("name", "jaronnie".to_string());

    // send
    let res = client.post("http://127.0.0.1:9090/post")
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(res)
}

#[tokio::main]
async fn main() {
    match get().await {
        Ok(resp) => println!("{:?}", resp),
        Err(error) => println!("{:?}", error),
    }

    match post().await {
        Ok(resp) => println!("{:?}", resp),
        Err(error) => println!("{:?}", error),
    }
}
