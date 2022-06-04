FROM rust:slim

ADD . /src
WORKDIR /src

RUN cargo build --release

ENTRYPOINT [ "/src/target/release/server" ]
