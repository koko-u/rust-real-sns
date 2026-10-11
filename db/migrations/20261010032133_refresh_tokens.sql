-- migrate:up
CREATE TABLE "refresh_tokens"
(
    "id"          uuid        NOT NULL DEFAULT uuidv7(),
    "session_id"  uuid        NOT NULL,
    "token_hash"  bytea       NOT NULL,
    "created_at"  timestamptz NOT NULL DEFAULT now(),
    "expires_at"  timestamptz NOT NULL,
    "consumed_at" timestamptz,
    "revoked_at"  timestamptz,
    CONSTRAINT "refresh_tokens_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "refresh_token_session_fkey" FOREIGN KEY ("session_id") REFERENCES "sessions" ("id") ON DELETE CASCADE,
    CONSTRAINT "refresh_token_hash" UNIQUE ("token_hash")
);

CREATE INDEX "refresh_tokens_session_id_idx" ON "refresh_tokens" ("session_id");

CREATE UNIQUE INDEX "refresh_tokens_active_session_idx"
    ON "refresh_tokens" ("session_id")
    WHERE "consumed_at" IS NULL
        AND "revoked_at" IS NULL;

-- migrate:down
DROP TABLE "refresh_tokens";
