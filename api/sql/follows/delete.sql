-- $1: me
-- $2: target
DELETE
FROM "follows"
WHERE "follower_id" = $1::uuid
  AND "followee_id" = $2::uuid