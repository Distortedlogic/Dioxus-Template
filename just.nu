export def setup [] {
  cargo install dioxus-cli@0.6.0-alpha.4
}

export def services [] {
  if not (which podman-compose | is-empty) {
    podman-compose down
    podman-compose up --build -d
  } else if not (which docker-compose | is-empty) {
    docker-compose down
    docker-compose up --build -d
  } else {
    error make { msg: "Neither podman-compose nor docker-compose found. Please install one of them." }
  }
}

export def tailwind [project: string, --watch(-w)] {
  let input_path = $'./($project)/input.css'
  let output_path =  $'./($project)/assets/tailwind.css'
  let config_path =  $'./($project)/tailwind.config.js'
  
  if $watch {
    npx tailwindcss -i $input_path -o $output_path -c $config_path --watch
  } else {
    npx tailwindcss -i $input_path -o $output_path -c $config_path
  }
}

export def runner [project: string, kind: string@get_kinds] {
  match $kind {
    "server" => { cargo run --features server -p $project }
    "desktop" => { cargo run --features desktop -p $project }
    "web" => { dx serve --features web --platform web -p $project }
    "mobile" => { dx serve --features mobile --platform mobile -p $project }
  }
}

export def get_kinds [] {
  ["server", "desktop", "web", "mobile"]
}

export def add_targets [] {
  # Android targets
  rustup target add aarch64-linux-android      # 64-bit ARM Android
  rustup target add armv7-linux-androideabi    # 32-bit ARM Android
  rustup target add x86_64-linux-android       # 64-bit x86 Android
  rustup target add i686-linux-android         # 32-bit x86 Android
  # iOS targets
  rustup target add aarch64-apple-ios          # 64-bit ARM iOS
  rustup target add x86_64-apple-ios          # iOS Simulator
  # WebAssembly targets
  rustup target add wasm32-unknown-unknown    # Pure WASM
  rustup target add wasm32-wasi              # WASM with system interface
  # Windows targets
  rustup target add x86_64-pc-windows-msvc    # 64-bit Windows MSVC
  rustup target add i686-pc-windows-msvc      # 32-bit Windows MSVC
  rustup target add x86_64-pc-windows-gnu     # 64-bit Windows GNU
  rustup target add i686-pc-windows-gnu       # 32-bit Windows GNU
  # macOS targets
  rustup target add x86_64-apple-darwin       # 64-bit Intel macOS
  rustup target add aarch64-apple-darwin      # 64-bit ARM macOS (Apple Silicon)
  # Linux targets
  rustup target add x86_64-unknown-linux-gnu   # 64-bit Linux (GNU)
  rustup target add i686-unknown-linux-gnu     # 32-bit Linux (GNU)
  rustup target add aarch64-unknown-linux-gnu  # 64-bit ARM Linux (GNU)
  rustup target add armv7-unknown-linux-gnueabihf  # 32-bit ARM Linux (GNU)
}

def main [] {}