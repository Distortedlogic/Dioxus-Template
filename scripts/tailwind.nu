#!/usr/bin/env nu

def main [
    project: string,  # Project name in workspace
    --watch(-w)      # Optional watch flag
] {
    cd $project
    let input_path = "./input.css"
    let output_path = "./public/tailwind.css"
    let config_path = "./tailwind.config.js"
    
    if $watch {
        npx tailwindcss -i $input_path -o $output_path -c $config_path --watch
    } else {
        npx tailwindcss -i $input_path -o $output_path -c $config_path
    }
}