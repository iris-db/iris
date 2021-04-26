FROM rust:1.43 as builder

WORKDIR build

COPY database .

RUN rustup install nightly
RUN rustup default nightly

RUN cargo build --release

FROM debian:buster-slim

WORKDIR callistodb

COPY --from=builder build/target/release/callistodb .

CMD ["./callistodb"]
