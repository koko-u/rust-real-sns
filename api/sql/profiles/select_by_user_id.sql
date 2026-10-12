-- $1: user_id
WITH "user_row" AS (SELECT "id", "username", "email"
                    FROM "users" AS "U"
                    WHERE "id" = $1::uuid)
SELECT "U"."id",
       "U"."username",
       "U"."email",
       "P"."display_name",
       "P"."biography",
       "P"."avatar_key",
       "P"."website_url",
       "P"."location"
FROM "user_row" AS "U"
         LEFT OUTER JOIN
     "profiles" AS "P"
     ON
         "U"."id" = "P"."user_id"

