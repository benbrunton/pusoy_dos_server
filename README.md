# Pusoy Dos
A multiplayer [Pusoy Dos](https://en.wikipedia.org/wiki/Pusoy_dos) server written in [Rust](https://www.rust-lang.org).

## Prerequisites
- [Docker](https://www.docker.com)

## Build
- Checkout source
- `make docker-build` 
- `make docker-run`

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

- [ ] - Prevent passing on empty table
- [ ] - multiple decks
- [ ] - Fix datetimes in DB
 - [ ] - time to begin game
 - [ ] - User creation datetime
 - [ ] - Game created datetime
- [ ] - Show Played Jokers
- [ ] - Card count Joker bug
- [ ] - Admin process
- [ ] - CSRF protection
- [ ] - User settings/profile
- [ ] - VM security audit ( close some ports etc )
- [ ] - Private groups/games
- [ ] - Private leaderboard
- [ ] - additional auth providers - { Google, Twitter, Github, Reddit }
- [ ] - Social sharing features
- [ ] - Nice Move! even when move is invalid
- [ ] - Additional stats
- [ ] - Redirect to homepage when user logged out
- [ ] - End game screens
- [ ] - max players auto start
- [ ] - optimise sessions
- [ ] - error pages
- [ ] - deployment/pipeline improvements
- [ ] - client side move validation
- [ ] - move assistance

## Contributing
Contributions are encouraged and welcome via Pull Request. 
Hit me up via an issue for suggestions or help.
