-- $1: user_id
WITH "target_user" AS (SELECT "id"
                       FROM "users"
                       WHERE "id" = $1::uuid)
SELECT "U"."id",
       "U"."username",
       "U"."email",
       "P"."display_name",
       "P"."biography",
       "P"."avatar_key",
       "P"."website_url",
       "P"."location"
FROM "target_user" AS "TU"
         INNER JOIN
     "follows" AS "F"
     ON
         "TU"."id" = "F"."followee_id"
         INNER JOIN
     "users" AS "U"
     ON
         "F"."follower_id" = "U"."id"
         LEFT OUTER JOIN
     "profiles" AS "P"
     ON
         "U"."id" = "P"."user_id";