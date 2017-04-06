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

## Roadmap

- [ ] - card count joker bug
- [ ] - auto pass when logged in bug
- [ ] - update number of decks after game creation
= [ ] - show when user wins hand
- [ ] - display number of decks on game list
- [ ] - display time left in time limit games
- [ ] - nice move! even when move is invalid
- [ ] - user settings/profile
- [ ] - max players auto start
- [ ] - private groups/games
- [ ] - private leaderboard
- [ ] - csrf protection and other post validation
- [ ] - collect email addresses
- [ ] - additional auth providers - { google, twitter, github, reddit }
- [ ] - social sharing features
- [ ] - additional stats
- [ ] - redirect to homepage when user logged out
- [ ] - end game screens
- [ ] - vm security audit ( close some ports etc )
- [ ] - optimise sessions
- [ ] - error pages
- [ ] - deployment/pipeline improvements
- [ ] - admin process
- [ ] - client side move validation
- [ ] - move assistance

## Contributing
Contributions are encouraged and welcome via Pull Request.
Hit me up via an issue for suggestions or help.
