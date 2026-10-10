SELECT "id", "name"
FROM "product_brands"
WHERE "id" = $1::uuid;