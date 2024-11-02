set shell := ["nu", "-c"]
set export
set dotenv-load

default:
  @just --choose --justfile {{justfile()}}

setup:
  just install
  just services
  @echo "Open three new terminals and run the following in each, just tailwind, just runner server, just runner desktop"

install:
  nu "scripts/install.nu"

services:
  nu "scripts/services.nu"

tailwind project="alpha-omega" watch="false":
  nu "scripts/tailwind.nu" {{project}} {{if watch == "true" { "--watch" } else { "" } }}

runner project="alpha-omega" target="desktop":
  nu "scripts/runner.nu" {{project}} {{target}}

add-targets:
  nu "scripts/add-targets.nu"