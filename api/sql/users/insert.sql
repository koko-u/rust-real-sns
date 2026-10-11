INSERT INTO "users" ("username", "email", "password_hash")
VALUES ($1::varchar, $2::varchar, $3::varchar)
RETURNING "id", "username", "email";