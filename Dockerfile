FROM debian:stretch
MAINTAINER Ben Brunton "ben.b.brunton@gmail.com"

ENV DEBIAN_FRONTEND=noninteractive
EXPOSE 3000

RUN apt-get update 
RUN apt-get install -y curl \
                       file \
                       sudo \
                       gcc \
                       libssl1.0-dev \
                       make \
                       psmisc \
                       gnupg \
                       software-properties-common

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y

SHELL ["/bin/bash", "-c"]

RUN curl -sL https://deb.nodesource.com/setup_9.x | sudo -E bash -
RUN apt-get install -y nodejs

RUN npm install -g stylus

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /project

CMD ["tail", "-F", "-n0", "/etc/hosts"]
