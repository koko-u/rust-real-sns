UPDATE "refresh_tokens"
SET "consumed_at" = $2::timestamptz
WHERE "id" = $1::uuid