use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;

async fn size_of_page(url: &str) -> Result<usize>{
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())

}

#[tokio::main]
async fn main(){
    let urls: [&str; 4] = [
        "https://www.rust-lang.org/".to_string(),
        "https://www.rust-lang.org/en-US/".to_string(),
        "https://www.rust-lang.org/zh-CN/".to_string(),
        "https://www.rust-lang.org/zh-TW/".to_string(),
    ];

    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> = urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
}