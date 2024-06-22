# syntax=docker.io/docker/dockerfile:1

# dockerfile just for build
FROM rust:1.79-bookworm

WORKDIR /app

COPY . /app

RUN ./build.sh && ./build.sh create_source

CMD [ "/bin/bash" ]
