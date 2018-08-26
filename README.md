# Pusoy Dos
A multiplayer [Pusoy Dos](https://en.wikipedia.org/wiki/Pusoy_dos) server written in [Rust](https://www.rust-lang.org).

## Prerequisites
- [Docker](https://www.docker.com)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [MiniKube](https://kubernetes.io)

## Configure
- Open project/config/app_config.tmpl, add any required secret keys, and save it as project/config/app_config.toml

## Build Server
- Checkout source
- `make ready-dev`
- `make go` (This command relies on the MYSQL container being ready. If it doesn't work then wait a few seconds and try again).
- Navigate to http://localhost:3010

Additional commands can be found in Makefile

## Tech
- [rust](https://www.rust-lang.org)
 - [gotham framework](https://gotham.rs/)
 - [tera templates](https://github.com/Keats/tera)
- [vue.js](https://vuejs.org)
- [pure.css](http://purecss.io/)
- [stylus](http://stylus-lang.com/)
- [mysql](https://www.mysql.com/)
- [docker](https://www.docker.com/)

## Contributing
Contributions are encouraged and welcome via Pull Request.
Hit me up via an issue for suggestions or help.
