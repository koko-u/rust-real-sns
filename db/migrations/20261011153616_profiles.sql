-- migrate:up
CREATE TABLE "profiles"
(
    "user_id"      uuid         NOT NULL,
    "display_name" varchar(255) NOT NULL,
    "biography"    varchar(4000),
    "avatar_key"   varchar(1024),
    "website_url"  varchar(2000),
    "location"     VARCHAR(4000),

    created_at     TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "profiles_pkey" PRIMARY KEY ("user_id"),
    CONSTRAINT "profiles_user_fkey" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);


CREATE TRIGGER "profile_update_at_tgr"
    BEFORE UPDATE
    ON "profiles"
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");


-- migrate:down
DROP TABLE "profiles";
