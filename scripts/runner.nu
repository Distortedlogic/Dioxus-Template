#!/usr/bin/env nu

def main [project: string, target: string@get_targets] {
    cd $project
    match $target {
        "server" => { cargo run --features server }
        "desktop" => { cargo run --features desktop }
        "web" => { dx serve --features web }
        "mobile" => { dx serve --features mobile }
    }
}

def get_targets [] {
    ["server", "desktop", "web", "mobile" ]
}