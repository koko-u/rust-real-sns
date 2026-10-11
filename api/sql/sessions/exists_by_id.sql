SELECT EXISTS (SELECT 1
               FROM "sessions"
               WHERE "id" = $1::uuid) AS "exists!";