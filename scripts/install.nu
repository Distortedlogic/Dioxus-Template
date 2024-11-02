#!/usr/bin/env nu

if not (which dx | is-empty) {
    print "Installing dioxus-cli..."
    cargo install dioxus-cli@0.6.0-alpha.4
} else {
    print "dioxus-cli is already installed"
}
