SELECT EXISTS (SELECT 1
               FROM "product_brands"
               WHERE "name" = $2::varchar
                 AND "id" <> $1::uuid) AS "exists!";