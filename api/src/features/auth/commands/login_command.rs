#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}
