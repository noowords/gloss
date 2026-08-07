#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}
