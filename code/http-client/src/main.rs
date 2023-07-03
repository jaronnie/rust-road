use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;


async fn get() -> Result<HashMap<String, String>, reqwest::Error>{
    Ok(reqwest::get("http://127.0.0.1:9090/json").await?.json::<HashMap<String, String>>().await?)
}

async fn post() -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("name", "jaronnie".to_string());

    // 发起post请求并返回
    Ok(client.post("http://127.0.0.1:9090/post").headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
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
