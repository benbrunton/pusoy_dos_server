# Pusoy Dos
A multiplayer [Pusoy Dos](https://en.wikipedia.org/wiki/Pusoy_dos) server written in [Rust](https://www.rust-lang.org).

## Prerequisites
- [Docker](https://www.docker.com)
- [npm](https://www.npmjs.com/)
 - stylus `npm install -g stylus`

## Configure
- Open project/config/app_config.tmpl, add the facebook app ID and Secret keys, and save it as project/config/app_config.toml

## Build Server
- Checkout source
- `make docker-build`
- `make docker-run`
- `make sh`
- `cargo run`

## Build Client
- `cd project`
- `make js`
- `make css`

Navigate to http://localhost:3010

## Tech
- [rust](https://www.rust-lang.org)
 - [iron framework](http://ironframework.io/)
 - [tera templates](https://github.com/Keats/tera)
- [vue.js](https://vuejs.org)
- [pure.css](http://purecss.io/)
- [stylus](http://stylus-lang.com/)
- [mysql](https://www.mysql.com/)
- [docker](https://www.docker.com/)

## Contributing
Contributions are encouraged and welcome via Pull Request.
Hit me up via an issue for suggestions or help.
