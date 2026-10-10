DELETE
FROM "product_brands"
WHERE "id" = $1::uuid
RETURNING "id", "name";