# watch server
watch_leptos: kill
    cargo leptos watch

# Watch webserver
watch: kill
    @sleep 2
    cargo watch -q -c -w service/ -w entity/ -x "run -p picktheday"

test: wait_for_test_db cargo_test stop_test_db

cargo_test:
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
    leptosfmt service
    cargo fmt --all
    cargo fix --lib --allow-dirty -p picktheday
    cargo fix --lib --allow-dirty -p entity

build:
    cargo leptos build --release

tailwind_watch:
    cd service && npx tailwindcss -i ./style/input.css -o ../public/main.css --watch
tailwind:
    cd service && npx tailwindcss -i ./style/input.css -o ../public/main.css

dependencies:
    rustup target add wasm32-unknown-unknown
    cargo install sea-orm-cli@1.0.0-rc.5
    cd service && npm install

start_db:
    docker compose up -d postgres

start_test_db:
    docker compose up -d test-postgres

stop_db:
    docker compose down

stop_test_db:
    docker compose down test-postgres

restart_db: stop_db start_db

status_db:
    @docker ps --filter "name=postgres" --filter "status=running" --format "{{{{.Names}} is running on port {{{{.Ports}}"

wait_for_db: start_db
    @until docker exec -it postgres pg_isready -U dev > /dev/null; do \
      echo "Waiting for postgres..."; \
      sleep 2; \
    done
    @echo "PostgreSQL is ready on port 5432!"

wait_for_test_db: start_test_db
    @until docker exec -it test-postgres pg_isready -U dev > /dev/null; do \
      echo "Waiting for postgres..."; \
      sleep 2; \
    done

reset_db: stop_db
    docker volume rm picktheday_dev_db

connect_db: wait_for_db
    @PGPASSWORD=devpassword psql -h localhost -p 5432 -U dev -d devdb