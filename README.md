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

## Todo
- [ ] - show when player has won in game
- [ ] - Round history ( moved, passed )
- [ ] - move timeout (auto-pass)
- [ ] - Nice Move! even when move is invalid
- [ ] - additional auth providers - { Google, Twitter, Github, Reddit }
- [ ] - Nice front page & additional info
- [ ] - improved game lobby menus
- [ ] - game rule options
- [ ] - multiple decks
- [ ] - SSL on play.benbru.com
- [ ] - Store sessions in redis
- [ ] - Additional performance audit and improvements
- [x] - real time updates
- [ ] - Tighten up security around all endpoints { permissions middleware? }
- [ ] - deployment
- [ ] - display datetime of interactions
- [ ] - error pages

## Backlog
- [ ] - private games
- [ ] - Social sharing features
- [ ] - client side move validation
- [ ] - help with moves
- [ ] - user profiles
- [ ] - mobile apps
- [ ] - CLI application
- [ ] - Changelog from github

## Contributing
Contributions are encouraged and welcome via Pull Request. 
Hit me up via an issue for suggestions or help.
