SELECT EXISTS (SELECT 1
               FROM "sessions"
               WHERE "id" = $1::uuid
                 AND "expires_at" > now()
                 AND "revoked_at" IS NULL) AS "exists!";