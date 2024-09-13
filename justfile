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

dependencies:
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
    rustup target add wasm32-unknown-unknown
    sudo apt-get install sqlite3 libsqlite3-dev