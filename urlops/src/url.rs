/// Base URL of discord API
const BASE_URL: &str = "https://discord.com/api";

pub struct Url {
    /// URL literal
    url: String,
    /// Is URL end.
    /// If the URL is end adding sub-path will panic.
    is_end: bool,
    user_id: &'static str,
    // token: &'static str,
}

impl Url {
    pub fn new() -> Self {
        Url {
            url: BASE_URL.to_string(),
            is_end: false,
            // token: "",
            user_id: "",
        }
    }

    pub fn from(user_id: &'static str) -> Self {
        Url {
            url: BASE_URL.to_string(),
            is_end: false,
            // token,
            user_id,
        }
    }

    pub fn as_string(&self) -> String {
        String::from(&self.url)
    }

    fn add_path(&mut self, sub: &'static str) -> &mut Self {
        if self.is_end {
            panic!("Adding path after query or something else is not allowed.")
        }

        self.url += "/";
        self.url += sub;

        self
    }

    fn add_query(&mut self, key: &'static str, value: &'static str) -> &mut Self {
        if self.is_end {
            // This means that precedent query exists.
            self.url += "&";
        } else {
            self.is_end = true;
        }

        self.url += "?";
        self.url += key;
        self.url += "=";
        self.url += value;

        self
    }

    pub fn get_url(&mut self, url_type: UrlType) -> String {
        match url_type {
            UrlType::Gateway => self.add_path("gateway").as_string(),
            UrlType::GatewayBot => self.add_path("gateway").add_path("bot").as_string(),
        }
    }
}

pub enum UrlType {
    Gateway,
    GatewayBot,
}

#[cfg(test)]
mod test {
    use super::*;

    const USER_ID: &'static str = "1234567890";
    // const TOKEN: &'static str = "THIS_IS_MY_TOKEN";

    #[test]
    fn empty_url_should_have_discord_url() {
        let url = Url::new();
        assert!(url.as_string().eq(BASE_URL))
    }

    #[test]
    fn gateway_url() {
        let mut url = Url::new();
        assert!(url
            .get_url(UrlType::Gateway)
            .eq(&format!("{}/gateway", BASE_URL)))
    }
}
