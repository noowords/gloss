set shell := ["powershell.exe", "-NoProfile", "-NoLogo", "-Command"]

dev action="":
    {{ if action == "build" { "docker compose -f compose.dev.yml build" } \
       else if action == "down" { "docker compose -f compose.dev.yml down" } \
       else if action == "restart" { "docker compose -f compose.dev.yml restart" } \
       else { "docker compose -f compose.dev.yml up" } }}

prod action="":
    {{ if action == "build" { "docker compose -f compose.prod.yml build" } \
       else if action == "down" { "docker compose -f compose.prod.yml down" } \
       else if action == "restart" { "docker compose -f compose.prod.yml restart" } \
       else { "docker compose -f compose.prod.yml up -d" } }}

logs:
    docker compose logs -f

ps:
    docker compose ps

build:
    cargo build

run:
    cargo run

test:
    cargo test

fmt:
    cargo fmt

clippy:
    cargo clippy --all-targets -- -D warnings

clean:
    cargo clean