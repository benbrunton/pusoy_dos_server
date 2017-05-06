FROM debian:jessie
MAINTAINER Ben Brunton "ben.b.brunton@gmail.com"

ENV DEBIAN_FRONTEND=noninteractive

EXPOSE 3000

RUN apt-get update 
RUN apt-get install -y curl \
                       file \
                       sudo \
                       gcc \
                       libssl-dev \
                       mysql-server \
                       mysql-client
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y

SHELL ["/bin/bash", "-c"]


ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /project

CMD ["cargo", "build"]
