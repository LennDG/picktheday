# watch server
watch: kill
    cargo leptos watch

migrate:
    diesel migration run

new_migration name:
    diesel migration generate {{name}}

zellij:
    zellij --layout zellij-layout.kdl

kill: 
    -lsof -i :3000 | awk 'NR==2 {print $2}' | xargs kill
    -lsof -i :3001 | awk 'NR==2 {print $2}' | xargs kill

fix:
    cargo fmt --all
    cargo fix --lib --allow-dirty --features ssr -p picktheday 
    cargo fix --lib --allow-dirty --features hydrate -p picktheday 

dependencies:
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
    rustup target add wasm32-unknown-unknown
    sudo apt-get install libpq-dev

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