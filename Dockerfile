FROM alpine:3.10.2

WORKDIR /shrtn

RUN apk add --no-cache rust cargo --repository http://dl-cdn.alpinelinux.org/alpine/edge/community
RUN cargo install cargo-build-deps

# hacky for caching build dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" >> src/main.rs
RUN cargo build-deps --release

COPY . .

RUN apk add --no-cache postgresql-dev
RUN cargo build --release

# make image smaller
RUN apk del rust cargo && \
    apk add libgcc

ENV RUST_BACKTRACE=full
CMD ./target/release/shrtn
