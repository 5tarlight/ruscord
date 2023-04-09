use std::{collections::HashMap, future::Future};

use urlops::url::Url;

pub async fn get_url(url: Url) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let res = reqwest::get(url.as_string())
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(res)
}

pub fn unwrap_future(
    future: impl Future<Output = Result<HashMap<String, String>, Box<dyn std::error::Error>>>,
) -> Result<HashMap<String, String>, ()> {
    let res = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();
    if let Ok(runtime) = res {
        Ok(runtime.block_on(async { future.await.unwrap() }))
    } else {
        Err(())
    }
}
