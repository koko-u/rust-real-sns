#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UserRow {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}
