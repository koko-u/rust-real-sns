SELECT EXISTS (SELECT 1
               FROM "product_brands"
               WHERE "name" = $1::varchar) AS "exists!";