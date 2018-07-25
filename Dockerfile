FROM ekidd/rust-musl-builder AS rust-build-env

ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

FROM alpine

RUN apk add --no-cache ca-certificates
COPY --from=rust-build-env /src/target/x86_64-unknown-linux-musl/release/discord-irasutoya /usr/bin/
