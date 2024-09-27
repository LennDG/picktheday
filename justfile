# watch server
watch: kill
    cargo leptos watch

test:
    cargo test --all-features -- --nocapture

migrate:
    sea-orm-cli migrate up

new_migration name:
    sea-orm-cli migrate generate {{name}}

generate_entity table:
    cp entity/src/lib.rs tmp
    sea-orm-cli generate entity -o entity/src --date-time-crate time --lib --tables {{table}}
    @-diff -u entity/src/lib.rs tmp > changes.patch
    @patch entity/src/lib.rs changes.patch
    @rm changes.patch tmp

zellij:
    zellij --layout zellij-layout.kdl

kill: 
    -lsof -i :3000 | awk 'NR==2 {print $2}' | xargs kill
    -lsof -i :3001 | awk 'NR==2 {print $2}' | xargs kill

fix:
    cargo fmt --all
    cargo fix --lib --allow-dirty --features ssr -p picktheday 
    cargo fix --lib --allow-dirty --features hydrate -p picktheday 

tailwind_watch:
    cd service && npx tailwindcss -i ./style/input.css -o ./style/output.css --watch

dependencies:
    rustup target add wasm32-unknown-unknown
    cargo install sea-orm-cli@1.0.0-rc.5

start_db:
    docker compose up -d postgres

stop_db:
    docker compose down

restart_db: stop_db start_db

status_db:
    @docker ps --filter "name=postgres" --filter "status=running" --format "{{{{.Names}} is running on port {{{{.Ports}}"

test_db: start_db
    @until docker exec -it postgres pg_isready -U dev > /dev/null; do \
      echo "Waiting for postgres..."; \
      sleep 2; \
    done
    @echo "PostgreSQL is ready on port 5432!"

reset_db:
    docker volume rm picktheday_dev_db

connect_db: test_db
    @PGPASSWORD=devpassword psql -h localhost -p 5432 -U dev -d devdb