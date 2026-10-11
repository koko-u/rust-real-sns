SELECT EXISTS (SELECT 1
               FROM "users"
               WHERE "email" = $1::varchar) AS "exists!";