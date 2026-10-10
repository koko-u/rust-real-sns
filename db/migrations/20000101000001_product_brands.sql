-- migrate:up
CREATE TABLE product_brands
(
    "id"         uuid         NOT NULL DEFAULT uuidv7(),
    "name"       varchar(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "product_brand_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "product_brand_name_unique" UNIQUE ("name")
);

CREATE INDEX "product_brand_name_idx_like" ON product_brands USING gin ("name" gin_trgm_ops);

CREATE TRIGGER "product_brand_update_at_tgr"
    BEFORE UPDATE
    ON product_brands
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "product_brands";
