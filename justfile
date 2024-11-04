set shell := ["nu", "-c"]
set export
set dotenv-load

default:
  @just --choose --justfile {{justfile()}}

setup:
  nu -c "source just.nu; setup"

add-targets:
  nu -c "source just.nu; add_targets"

services:
  nu -c "source just.nu; services"

tailwind project="alpha-omega" watch="false":
  nu -c "source just.nu; tailwind {{project}} {{if watch == "true" { "--watch" } else { "" } }}"

runner project="alpha-omega" target="desktop":
  nu -c "source just.nu; runner {{project}} {{target}}"

gen-db-data:
  cargo run -p cli -- generate-users

install-postgres:
  sudo dnf install -y postgresql-devel postgresql-server postgresql-contrib