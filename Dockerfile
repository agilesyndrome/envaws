FROM syndromeos:latest

RUN curl -L -o rusty.sh https://sh.rustup.rs -sSf \
 && chmod +x ./rusty.sh \
 && ./rusty.sh -y \
 && . $HOME/.cargo/env

WORKDIR /app

ADD . /app

ENV OPENSSL_DIR=/usr/openssl

RUN . $HOME/.cargo/env \
 && cargo build
