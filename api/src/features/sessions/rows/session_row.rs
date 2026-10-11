#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct SessionRow {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expires_at: time::OffsetDateTime,
}
