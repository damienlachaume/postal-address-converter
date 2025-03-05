install:
    cargo build --release
    export PATH="$PATH:$(pwd)/target/release"
