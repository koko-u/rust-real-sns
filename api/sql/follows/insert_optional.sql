-- $1: follower_id
-- $2: followee_id
INSERT INTO "follows" ("follower_id", "followee_id")
VALUES ($1::uuid, $2::uuid)
ON CONFLICT ("follower_id", "followee_id") DO NOTHING 