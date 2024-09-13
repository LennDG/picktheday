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