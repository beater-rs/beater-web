FROM rust:slim

RUN apt-get update && apt-get install -y perl

ADD . /src
WORKDIR /src

RUN cargo build --release

ENTRYPOINT [ "/src/target/release/server" ]
