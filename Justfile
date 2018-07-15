run:
    @cargo build
    @sudo setcap cap_net_raw+ep ./target/debug/dPoSL
    @./target/debug/dPoSL

fmt:
    @cargo fmt
