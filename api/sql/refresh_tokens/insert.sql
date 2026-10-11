INSERT INTO "refresh_tokens" ("session_id", "token_hash", "expires_at")
VALUES ($1::uuid, $2::bytea, $3::timestamptz)