/// Base URL of discord API
const BASE_URL: &str = "https://discord.com/api";

/// Discord api URL struct.
/// This struct is utility for creating dynamic discord api URL.
pub struct Url {
    /// URL literal
    url: String,

    /// Is URL end.
    /// If the URL is end adding sub-path will panic.
    is_end: bool,

    /// Discord application id. This is not author's discord id
    #[allow(unused)]
    user_id: &'static str,
    // token: &'static str,
}

impl Url {
    /// Create new empty URL instance.
    /// This will make url with empty `user_id`.
    /// If you build URL which requires user_id with instance made by this fn,
    /// it will words in an unintended way.
    pub fn new() -> Self {
        Url {
            url: BASE_URL.to_string(),
            is_end: false,
            // token: "",
            user_id: "",
        }
    }

    /// Create new empty URL instance with user id.
    /// If you want to build some URL which requries `user_id`,
    /// this fn is perfect method.
    pub fn from(user_id: &'static str) -> Self {
        Url {
            url: BASE_URL.to_string(),
            is_end: false,
            // token,
            user_id,
        }
    }

    /// Build complete URL.
    ///
    /// # Example
    /// ```
    /// use urlops::url::Url;
    ///
    /// let url = Url::new().as_string();
    /// assert!(url.eq("https://discord.com/api"))
    /// ```
    pub fn as_string(&self) -> String {
        String::from(&self.url)
    }

    /// This will append new path to URL.
    ///
    /// # Panics
    /// Adding new path after append query is not allowed.
    /// Because this may make some URL such as `http://test.com?id=1/oh-no`.
    ///
    /// ```ignore
    /// use urlops::url::Url;
    ///
    /// Url::new().add_query("id", "1").add_path("oh-no")
    /// ```
    fn add_path(&mut self, sub: &'static str) -> &mut Self {
        if self.is_end {
            panic!("Adding path after query or something else is not allowed.")
        }

        self.url += "/";
        self.url += sub;

        self
    }

    /// This function will append new query pair to URL.
    /// `?key=value` or `&key=value`.
    /// Also after using this fn, calling `add_path` is prohibited.
    #[allow(unused)]
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

    /// Make URL literal for certain proposes.
    /// Mostly recommended.
    pub fn get_url(&mut self, url_type: UrlType) -> &Self {
        match url_type {
            UrlType::Gateway => self.add_path("gateway"),
            UrlType::GatewayBot => self.add_path("gateway").add_path("bot"),
        }
    }
}

/// Pre-defined URL types.
pub enum UrlType {
    Gateway,
    GatewayBot,
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
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
