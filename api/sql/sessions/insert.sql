INSERT INTO "sessions" ("user_id", "expires_at")
VALUES ($1::uuid, $2::timestamptz)
RETURNING "id", "user_id", "expires_at"