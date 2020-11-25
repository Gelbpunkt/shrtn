# shrtn

shrtn is a blazingly fast, extension aware link shortener.

It is written in Rust using Actix for maximum performance and uses Redis for fast, persistent-if-you-want, storage.

## Installation

### Recommended (podman / docker)

```
podman build -t shrtn:latest .
podman pod create -n shrtn -p 4445:4445
podman run --rm --name redis -d --pod shrtn redis:alpine
podman run --rm --name shrtn-main --env-file .env --pod shrtn shrtn:latest
```

### Unrecommended (normal)

```
cargo build --release
export $(cat .env | xargs)
./target/release/shrtn
```

## License

AGPLv3+
