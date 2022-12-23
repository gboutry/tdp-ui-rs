# TDP-UI in Rust ?!?

This project is an experiment.

## Needed tools:

Trunk is needed to build / bundle / serve the website.

```bash
cargo install trunk
```

## Docker compose:

Setup Keycloak, Postgres, and tdp-server

```bash
docker compose -f env/compose.yml up -d
```

## Start the project

```bash
trunk serve --release --port 3000 crates/ui/index.html
```
