-- migrate:up
CREATE TABLE "follows"
(
    "follower_id" uuid NOT NULL,
    "followee_id" uuid NOT NULL,
    CONSTRAINT "follows_pkey" PRIMARY KEY ("follower_id", "followee_id"),
    CONSTRAINT "follows_follower_id_fkey" FOREIGN KEY ("follower_id") REFERENCES "users" ("id") ON DELETE CASCADE,
    CONSTRAINT "follows_followee_id_fkey" FOREIGN KEY ("followee_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE INDEX "follows_follower_id_idx" ON "follows" ("follower_id");
CREATE INDEX "follows_followee_id_idx" ON "follows" ("followee_id");

-- migrate:down
DROP TABLE "follows";
