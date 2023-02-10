FROM rust:1.67 AS video_push-builder
WORKDIR /usr/src

# caching in rust is a bitch.

#First, we copy in our cargo.toml files. 
COPY video_push/Cargo.toml video_push/Cargo.toml
COPY common/Cargo.toml common/Cargo.toml

#Then we create dummy files for our app and common to allow them to compile
RUN mkdir -p video_push/src common/src
RUN echo "fn main() {panic!(\"I'm not real\")}" > video_push/src/main.rs
RUN echo "pub fn lib() {}" > common/src/lib.rs

# build fake video_push. By doing this we build our dependencies
WORKDIR /usr/src/video_push
RUN cargo build --release
WORKDIR /usr/src

# remove fake video_push
RUN rm -rf video_push/src common/src

# copy in the real ones
COPY video_push/src video_push/src
COPY common/src common/src

# touch them so that cargo knows it has to rebuild
RUN touch common/src/lib.rs
RUN touch video_push/src/main.rs

WORKDIR /usr/src/video_push
# build
RUN cargo build --release
# install
RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=video_push-builder /usr/local/cargo/bin/video_push /app/video_push

EXPOSE 8080

ENTRYPOINT /app/video_push