run: build setperm
    ./target/debug/xeo

build:
    cargo build -q

setperm:
    for f in $(ls ./target/debug/xeo-*); do \
        sudo setcap cap_net_raw+ep $f; \
    done

fmt:
    cargo fmt

check:
    cargo check

test: build setperm
    cargo test -q

p port: build setperm
    ./target/debug/xeo -p {{port}}
