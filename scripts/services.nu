#!/usr/bin/env nu

if not (which podman-compose | is-empty) {
    podman-compose down
    podman-compose up --build -d
} else if not (which docker-compose | is-empty) {
    docker-compose down
    docker-compose up --build -d
} else {
    error make { msg: "Neither podman-compose nor docker-compose found. Please install one of them." }
}