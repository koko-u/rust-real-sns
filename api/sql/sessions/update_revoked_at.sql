UPDATE "sessions"
SET "revoked_at" = $2::timestamptz
WHERE "id" = $1::uuid