# Pusoy Dos
A multiplayer [Pusoy Dos](https://en.wikipedia.org/wiki/Pusoy_dos) server written in [Rust](https://www.rust-lang.org).

## Prerequisites
- Docker

## Build
- Checkout source
- `make docker-build` 
- `make docker-run`

## Tech
- [rust](https://www.rust-lang.org)
 - [iron framework](http://ironframework.io/)
 - [tera templates](https://github.com/Keats/tera)
- mysql
- docker

## Todo
- [ ] - implement jokers
- [ ] - implement reversals (configured at game creation)
- [ ] - stylesheets
- [ ] - real time updates
- [ ] - additional auth providers - { Google, Twitter, Github, Reddit }
- [ ] - game rule options
- [ ] - multiple decks
- [ ] - Nice front page & additional info
- [ ] - "Fork me on github"
- [ ] - move timeout (auto-pass)
- [ ] - SSL on play.benbru.com
- [ ] - deployment
- [ ] - display datetime of interactions
- [ ] - error pages
