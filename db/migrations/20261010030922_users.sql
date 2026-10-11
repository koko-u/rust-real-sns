-- migrate:up
CREATE TABLE "users"
(
    "id"            uuid         NOT NULL DEFAULT uuidv7(),
    "username"      varchar(255) NOT NULL,
    "email"         varchar(255) NOT NULL,
    "password_hash" text         NOT NULL,
    "created_at"    timestamptz  NOT NULL DEFAULT now(),
    "updated_at"    timestamptz  NOT NULL DEFAULT now(),
    CONSTRAINT "users_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "user_username_unique" UNIQUE ("username"),
    CONSTRAINT "user_email_unique" UNIQUE ("email")
);

CREATE TRIGGER "user_update_at_tgr"
    BEFORE UPDATE
    ON "users"
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "users";

