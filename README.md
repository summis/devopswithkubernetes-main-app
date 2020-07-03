# devopswithkubernetes-main-app

`Stamper` saves every five seconds timestamp to file `/var/log/main-app/timestamp.txt`. Run with
```
cargo run --manifest-path stamper/Cargo.toml
```
or with docker image `summila/main-app-stamper` or start with kubernetes.


`Server` generates random hash and serves it at http://localhost:8080 and prints timestamp that it read from file `/var/log/main-app/timestamp.txt` and hash every five seconds.
Run with
```
cargo run --manifest-path server/Cargo.toml
```
or with docker image `summila/main-app-server` or start with kubernetes.

