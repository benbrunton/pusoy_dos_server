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

- [ ] - Event Log
- [ ] - Fix datetimes in DB
- [ ] - Card count Joker bug
- [ ] - Leading player
- [ ] - CSRF protection
- [ ] - VM security audit



- [ ] - Private games
- [ ] - Private leaderboard
- [ ] - additional auth providers - { Google, Twitter, Github, Reddit }



- [ ] - Push notifications
- [ ] - move timeout (auto-pass)
- [ ] - multiple decks
- [ ] - Social sharing features
- [ ] - Round history ( moved, passed )
- [ ] - Nice Move! even when move is invalid
- [ ] - Additional stats
- [ ] - Redirect to homepage when user logged out
- [ ] - End game screens
- [ ] - improved game menus
 - [ ] - game lobby (including notifications etc)
 - [ ] - Join game
 - [ ] - create game
 - [ ] - Joker modal
- [ ] - game rule options
- [ ] - Store sessions in redis
- [x] - real time updates
- [ ] - deployment
- [ ] - display datetime of interactions
- [ ] - error pages
- [ ] - client side move validation
- [ ] - help with moves
- [ ] - user profiles
- [ ] - mobile apps
- [ ] - CLI application
- [ ] - Changelog from github
- [ ] - Integrations (slack?)

## Contributing
Contributions are encouraged and welcome via Pull Request. 
Hit me up via an issue for suggestions or help.
