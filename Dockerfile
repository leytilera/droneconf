FROM rust:buster as builder

RUN apt update && apt install -y libssl-dev

WORKDIR /usr/src/droneconf

COPY Cargo.toml ./
COPY src/ src/

RUN cargo build --release

FROM debian:buster

COPY --from=builder /usr/src/droneconf/target/release/droneconf /usr/bin

RUN apt update && apt install -y libssl1.1 dumb-init

VOLUME ["/data"]

ENTRYPOINT ["/usr/bin/dumb-init", "--", "/usr/bin/droneconf", "--config", "/data/config.toml"]