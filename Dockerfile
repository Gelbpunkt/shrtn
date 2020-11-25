FROM alpine:edge AS builder

RUN apk add --no-cache gcc musl-dev curl && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

WORKDIR /build

# hacky for caching build dependencies
COPY Cargo.toml Cargo.lock ./

RUN mkdir src/
RUN echo "fn main() { }" >> src/main.rs
RUN source $HOME/.cargo/env && \
    cargo build --release

RUN rm -f target/release/deps/shrtn*
COPY ./src ./source
RUN source $HOME/.cargo/env && \
    cargo build --release

FROM alpine:edge

COPY --from=builder /build/target/release/shrtn /usr/bin/shrtn

CMD /usr/bin/shrtn
