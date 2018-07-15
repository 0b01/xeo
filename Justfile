run:
    @cargo build
    @ \
    for f in $(ls ./target/debug/mdc-*); do \
        sudo setcap cap_net_raw+ep $f; \
    done
    @./target/debug/mdc

fmt:
    @cargo fmt

test:
    @cargo test
