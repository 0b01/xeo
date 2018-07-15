run: build setperm
    ./target/debug/mdc

build:
    cargo build -q

setperm:
    for f in $(ls ./target/debug/mdc-*); do \
        sudo setcap cap_net_raw+ep $f; \
    done

fmt:
    cargo fmt

test: build setperm
    cargo test -q
