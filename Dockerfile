FROM ekidd/rust-musl-builder AS rust-build-env

RUN sudo chown -R rust:rust /home/rust && \
  USER=rust cargo init 
ADD Cargo.toml Cargo.lock ./
RUN cargo build --release
ADD . ./
RUN rm /home/rust/src/target/x86_64-unknown-linux-musl/release/discord-irasutoya && \
  cargo build --release

FROM alpine

RUN apk add --no-cache ca-certificates
COPY --from=rust-build-env /home/rust/src/target/x86_64-unknown-linux-musl/release/discord-irasutoya /usr/bin/
