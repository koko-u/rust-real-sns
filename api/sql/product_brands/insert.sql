INSERT INTO "product_brands" ("name")
VALUES ($1::varchar)
ON CONFLICT ("name") DO NOTHING
RETURNING "id", "name";