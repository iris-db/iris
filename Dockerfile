FROM rust:1.43 as builder

WORKDIR build

COPY iris .

RUN rustup install nightly
RUN rustup default nightly

RUN cargo build --release

FROM debian:buster-slim

WORKDIR iris

COPY --from=builder build/target/release/iris .

CMD ["./iris"]
