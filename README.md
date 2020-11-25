# shrtn

shrtn is a blazingly fast, extension aware link shortener.

It is written in Rust using Actix for maximum performance and is cached. The backend is PostgreSQL.

## Installation

### Recommended (podman / docker)

```
podman build -t shrtn:latest .
podman pod create -n shrtn -p 4445:4445
podman run --rm --name psql -d --pod shrtn postgresql:13-alpine
podman exec -it psql ash
su postgres
createdb links
psql links <(curl -s schema_url)
exit
exit
podman run --rm --name shrtn-main --env-file .env --pod shrtn shrtn:latest
```

### Unrecommended (normal)

```
# setup db before
cargo build --release
export $(cat .env | xargs)
./target/release/shrtn
```

## License

AGPLv3+
