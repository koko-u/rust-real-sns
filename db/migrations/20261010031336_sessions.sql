-- migrate:up
CREATE TABLE sessions
(
    "id"         uuid        NOT NULL DEFAULT uuidv7(),
    "user_id"    uuid        NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "expires_at" timestamptz NOT NULL,
    "revoked_at" timestamptz,
    CONSTRAINT "sessions_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "sessions_user" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE INDEX "sessions_user_id_idx" ON "sessions" ("user_id");

-- migrate:down
DROP TABLE "sessions";
