#[allow(unused)]
pub struct Client {
    token: String,
    id: String,
    username: String,
    global_name: Option<String>,
    avatar: Option<String>,
    avatar_decoration: Option<String>,
    discriminator: u8,
    public_flags: i32,
    flags: i32,
    bot: bool,
    banner: Option<String>,
    banner_color: Option<String>,
    accent_color: Option<String>,
    bio: String,
    locale: String, // TODO : ex. us-US. Enum
    mfa_enabled: bool,
    premium_type: u8,
    linked_users: Vec<String>,
    email: Option<String>,
    verified: bool,
}
