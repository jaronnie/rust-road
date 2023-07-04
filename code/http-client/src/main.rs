use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;
use anyhow::{Context, Result};


async fn get() -> Result<HashMap<String, Value>, Box<dyn std::error::Error>>{
    let res = reqwest::get("http://127.0.0.1:9090/json")
        .await
        .context("请求错误")?
        .json::<HashMap<String, Value>>()
        .await
        .context("反序列化错误")?;
    Ok(res)
}

async fn post() -> Result<HashMap<String, Value>,Box<dyn std::error::Error>>{
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
        .await
        .context("post meet error")?
        .json::<HashMap<String, Value>>()
        .await
        .context("反序列化错误")?;
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
