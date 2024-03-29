FROM rust:1.39

WORKDIR /rca
ADD . .

RUN cargo clean
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

CMD ./target/release/rca
