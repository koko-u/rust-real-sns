#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignupCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}
