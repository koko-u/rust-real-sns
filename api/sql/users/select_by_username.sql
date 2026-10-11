SELECT "id", "username", "email", "password_hash"
FROM "users"
WHERE "username" = $1::varchar