workspace_root := justfile_directory()

_default:
    @just --list

# run rest api
api:
    @cargo run --package api --bin server --features api-doc

# watch rest api
api-dev:
    @cargo watch --clear --quiet --exec "run --package api --bin server --features api-doc"

# run angular spa
[working-directory("ui")]
ui-dev:
    pnpm start

# run api and spa
[parallel]
dev: api-dev ui-dev

# docker up
dup:
    @docker compose -f {{ workspace_root }}/docker/compose.yml up -d

# docker down
dwn:
    @docker compose -f {{ workspace_root }}/docker/compose.yml down

# dbmate rollback all
rollback-all:
    #!/usr/bin/env bash
    set -euo pipefail

    count=$(dbmate status | awk '/^Applied:/ { print $2 }')

    echo "Rolling back $count migration(s)..."

    for ((i = 0; i < count; i++)); do
        dbmate rollback
    done

# format rust code
api-fmt:
    @cargo +nightly fmt

# format typescript code
[working-directory("ui")]
ui-fmt:
    @pnpm run fmt

# format
[parallel]
fmt: api-fmt ui-fmt

# signup
signup user email pass:
    @xh POST :3000/api/auth/signup username={{ user }} email={{ email }} password={{ pass }}

# login
login user pass:
    @xh --session=/tmp/real-sns-cookie.json POST :3000/api/auth/login username={{ user }} password={{ pass }}

# refresh
refresh:
    @xh --verbose --session=/tmp/real-sns-cookie.json POST :3000/api/auth/refresh

# logout
logout:
    @xh --verbose --session=/tmp/real-sns-cookie.json POST :3000/api/auth/logout
