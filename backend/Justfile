set shell := ["powershell.exe", "-NoProfile", "-NoLogo", "-Command"]

up action="":
    {{ if action == "dev" { "docker compose -f compose.dev.yml up" } \
       else if action == "release" { "docker compose -f compose.prod.yml up -d" } \
       else { "docker compose -f compose.dev.yml up" } }}

build action="":
    {{ if action == "dev" { "docker compose -f compose.dev.yml build" } \
       else if action == "release" { "docker compose -f compose.prod.yml build" } \
       else { "docker compose -f compose.dev.yml build" } }}

restart action="":
    {{ if action == "dev" { "docker compose -f compose.dev.yml restart" } \
       else if action == "release" { "docker compose -f compose.prod.yml restart" } \
       else { "docker compose -f compose.dev.yml restart" } }}

down action="":
    {{ if action == "dev" { "docker compose -f compose.dev.yml down" } \
       else if action == "release" { "docker compose -f compose.prod.yml down" } \
       else { "docker compose -f compose.dev.yml down" } }}

logs:
    docker compose logs -f

ps:
    docker compose ps
