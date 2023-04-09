use std::{collections::HashMap, future::Future};

use urlops::url::Url;

/// Fetch `Url` instance.
/// This is `async` function and will return `Future<Result<HashMap<_,_>, _>>`.
/// You can use `unwrap_future` function in non-async (sync) function to wait for
/// future data input.
pub async fn fetch_url(url: &Url) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let res = reqwest::get(url.as_string())
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(res)
}

/// Wait for future.
/// This method is designed to await future in non-async (sync) functions.
/// This function will block sync function which is calling this function.
/// Also intended to use with `fetch_url` method.
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
