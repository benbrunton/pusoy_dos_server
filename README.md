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
- [ ] - UI design
- [ ] - real time updates
- [ ] - additional auth providers - { Google, Twitter, Github, Reddit }
- [ ] - move timeout (auto-pass)
- [ ] - Nice front page & additional info
- [ ] - game rule options
- [ ] - multiple decks
- [ ] - "Fork me on github"
- [ ] - SSL on play.benbru.com
- [ ] - deployment
- [ ] - display datetime of interactions
- [ ] - error pages
