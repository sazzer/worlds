# Build the code first
FROM rustlang/rust:nightly-buster-slim AS builder

WORKDIR /worlds

# First install any needed system dependencies
RUN rustc --version --verbose
RUN apt update -y && apt install -y pkg-config libssl-dev libclang-dev clang

# Download and compile the rust dependencies
RUN USER=worlds cargo init --bin --name worlds .
COPY Cargo.lock Cargo.toml /worlds/
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /worlds/target -name "*worlds*" | xargs rm -rf

# Now build the actual app
RUN rm -rf /worlds/src
COPY src /worlds/src

COPY migrations /worlds/migrations

RUN cargo build --release

# Next build a container with the build artifact but no code
FROM debian:buster-slim

RUN apt update -y && apt install -y openssl ca-certificates wget

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /worlds

COPY --from=builder /worlds/target/release/worlds_service /worlds/worlds
COPY docker/start.sh /worlds

ENV PORT=8000
ENV RUST_LOG=debug

CMD ["/worlds/start.sh"]
