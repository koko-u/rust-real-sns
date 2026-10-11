SELECT "RT"."id",
       "RT"."session_id",
       "S"."user_id"    AS "session_user_id",
       "S"."expires_at" AS "session_expires_at!",
       "S"."revoked_at" AS "session_revoked_at",
       "RT"."expires_at",
       "RT"."consumed_at",
       "RT"."revoked_at"
FROM "refresh_tokens" AS "RT"
         INNER JOIN
     "sessions" AS "S"
     ON
         "RT"."session_id" = "S"."id"
WHERE "RT"."token_hash" = $1::bytea