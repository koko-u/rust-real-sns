UPDATE "refresh_tokens"
SET "revoked_at" = $2::timestamptz
WHERE "session_id" = $1::uuid