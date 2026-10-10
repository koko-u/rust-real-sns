-- migrate:up
CREATE EXTENSION "uuid-ossp";
CREATE EXTENSION "pg_trgm";
CREATE EXTENSION "moddatetime";

-- migrate:down
DROP EXTENSION "uuid-ossp";
DROP EXTENSION "pg_trgm";
DROP EXTENSION "moddatetime";
