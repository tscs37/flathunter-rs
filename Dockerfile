FROM clux/muslrust AS build

RUN mkdir /build
COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock
COPY ./src /build/src
RUN cd /build && cargo build --release

FROM alpine:latest

RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/flathunter-rs /flathunter-rs

CMD ["/flathunter-rs"]