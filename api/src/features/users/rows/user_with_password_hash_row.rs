#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UserWithPasswordHashRow {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
