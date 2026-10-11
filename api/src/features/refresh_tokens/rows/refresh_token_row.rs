#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct RefreshTokenRow {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub session_user_id: uuid::Uuid,
    pub session_expires_at: time::OffsetDateTime,
    pub session_revoked_at: Option<time::OffsetDateTime>,
    pub expires_at: time::OffsetDateTime,
    pub consumed_at: Option<time::OffsetDateTime>,
    pub revoked_at: Option<time::OffsetDateTime>,
}
