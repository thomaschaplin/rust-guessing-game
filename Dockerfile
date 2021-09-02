FROM rust:latest as build

USER root

WORKDIR /rust-guessing-game

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM rust:latest

COPY --from=build /rust-guessing-game/target/release/rust-guessing-game .

CMD ["./rust-guessing-game"]
