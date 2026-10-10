WITH "params" ("name") AS (VALUES ($1::varchar)),
     "inserted" AS (
         INSERT INTO "product_brands" ("name")
             SELECT "name"
             FROM "params"
             ON CONFLICT ("name") DO NOTHING
             RETURNING "id", "name"),
     "united" AS (SELECT "PB"."id", "PB"."name"
                  FROM "product_brands" AS "PB"
                           INNER JOIN
                       "params" AS "P"
                       ON
                           "PB".name = "P"."name"
                  UNION
                  SELECT "id", "name"
                  FROM "inserted")
SELECT "id" AS "id!", "name" AS "name!"
FROM "united";