FROM rust:1.67 AS webapp-builder
WORKDIR /usr/src

# caching in rust is a bitch.

#First, we copy in our cargo.toml files. 
COPY webapp/Cargo.toml webapp/Cargo.toml
COPY common/Cargo.toml common/Cargo.toml

#Then we create dummy files for our app and common to allow them to compile
RUN mkdir -p webapp/src common/src
RUN echo "fn main() {panic!(\"I'm not real\")}" > webapp/src/main.rs
RUN echo "pub fn lib() {}" > common/src/lib.rs

# build fake webapp. By doing this we build our dependencies
WORKDIR /usr/src/webapp
RUN cargo build --release
WORKDIR /usr/src

# remove fake webapp
RUN rm -rf webapp/src common/src

# copy in the real ones
COPY webapp/src webapp/src
COPY common/src common/src

# touch them so that cargo knows it has to rebuild
RUN touch common/src/lib.rs
RUN touch webapp/src/main.rs

WORKDIR /usr/src/webapp
# build
RUN cargo build --release
# install
RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=webapp-builder /usr/local/cargo/bin/webapp /app/webapp

EXPOSE 8080

ENTRYPOINT /app/webapp