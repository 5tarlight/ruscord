/// Base URL of discord API
const BASE_URL: &str = "https://discord.com/api";

pub struct Url {
    /// URL literal
    url: String,
    /// Is URL end.
    /// If the URL is end adding sub-path will panic.
    is_end: bool,
}

impl Url {
    pub fn new() -> Self {
        Url {
            url: BASE_URL.to_string(),
            is_end: false,
        }
    }

    pub fn as_string(&self) -> String {
        String::from(&self.url)
    }

    fn add_path(&mut self, sub: &'static str) {
        if self.is_end {
            panic!("Adding path after query or something else is not allowed.")
        }

        self.url += "/";
        self.url += sub;
    }

    fn add_query(&mut self, key: &'static str, value: &'static str) {
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
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_url_should_have_discord_url() {
        let url = Url::new();
        assert!(url.as_string().eq(BASE_URL))
    }
}
