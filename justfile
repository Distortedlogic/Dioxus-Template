set shell := ["nu", "-c"]
set export
set dotenv-load

default:
  @just --choose --justfile {{justfile()}}

setup:
  cargo install dioxus-cli@0.6.0-alpha.4

services:
  podman-compose down
  podman-compose up --build -d

tailwind:
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css -c ./tailwind.config.js --watch

gen-db-data:
  cargo run -p cli -- generate-users

install-postgres:
  sudo dnf install -y postgresql-devel postgresql-server postgresql-contrib