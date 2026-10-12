-- $1: user_id
-- $2: display_name
-- $3: biography
-- $4: avatar_key
-- $5: website_url
-- $6: location

INSERT INTO "profiles" ("user_id",
                        "display_name",
                        "biography",
                        "avatar_key",
                        "website_url",
                        "location")
VALUES ($1::uuid,
        $2::varchar,
        $3::varchar,
        $4::varchar,
        $5::varchar,
        $6::varchar)
ON CONFLICT ("user_id") DO UPDATE
    SET "display_name" = $2::varchar,
        "biography"    = $3::varchar,
        "avatar_key"   = $4::varchar,
        "website_url"  = $5::varchar,
        "location"     = $6::varchar
