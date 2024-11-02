set shell := ["nu", "-c"]
set export
set dotenv-load

default:
  @just --choose --justfile {{justfile()}}

setup:
  nu "just.scripts.nu" "setup"

add-targets:
  nu just.scripts.nu add-targets

services:
  nu -c "source just.nu; services"

tailwind project="alpha-omega" watch="false":
  nu "just.scripts.nu" "tailwind" {{project}} {{if watch == "true" { "--watch" } else { "" } }}

runner project="alpha-omega" target="desktop":
  nu "just.scripts.nu" "runner" {{project}} {{target}}

