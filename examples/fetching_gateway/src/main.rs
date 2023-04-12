use ruscord::{
    ruscord_net::{fetch_url, unwrap_future},
    urlops::url::{Url, UrlType},
};

fn main() {
    let mut url = Url::new();
    url.get_url(UrlType::Gateway);
    let future = fetch_url(&url);
    let result = unwrap_future(future).unwrap();
    println!("{}", result.get("url").unwrap())
}
