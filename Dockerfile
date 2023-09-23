FROM rust:1.72.1 as build

COPY src/ /build/src
COPY Cargo.toml /build/Cargo.toml
COPY Cargo.lock /build/Cargo.lock

WORKDIR /build
RUN cargo build --release

CMD ["/build/target/release/aqi-alert-bot"]