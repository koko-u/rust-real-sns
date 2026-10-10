UPDATE "product_brands"
SET "name" = $2::varchar
WHERE "id" = $1::uuid
RETURNING "id", "name";