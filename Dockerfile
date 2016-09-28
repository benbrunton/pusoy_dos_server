FROM debian:jessie
MAINTAINER Ben Brunton "ben.b.brunton@gmail.com"

ENV DEBIAN_FRONTEND=noninteractive

EXPOSE 3000

RUN apt-get update 
RUN apt-get install -y curl file sudo gcc libssl-dev
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- -y

WORKDIR /project

CMD ["cargo", "run"]
