FROM rust as builder

ARG toolchainbuild
RUN apt-get update && \
  apt install -y git clang curl libssl-dev llvm libudev-dev

RUN echo "rust toolchain build: $toolchainbuild"
RUN rustup install $toolchainbuild &&\
  rustup default $toolchainbuild &&\
  rustup update $toolchainbuild &&\
  rustup target add wasm32-unknown-unknown --toolchain $toolchainbuild

COPY . .
RUN cargo build --release

FROM alpine:3.13 as app

COPY --from=builder /app/target/release/opportunity-standalone /usr/local/bin

RUN /usr/local/bin/opportunity-standalone --version

# USER standard
EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/opportunity-standalone"]