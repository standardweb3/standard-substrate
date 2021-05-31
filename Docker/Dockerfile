ARG RUST_IMAGE=rust:1.52-slim
ARG RUNTIME_IMAGE=alpine:3.13

FROM $RUST_IMAGE as planner
ARG toolchainbuild
WORKDIR /opt/app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN echo "rust toolchain build: $toolchainbuild"
RUN rustup default $toolchainbuild
RUN cargo install cargo-chef 

COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM $RUST_IMAGE as cacher
ARG toolchainbuild
WORKDIR /opt/app

RUN apt-get update && \
  apt install -y git clang curl libssl-dev llvm libudev-dev
RUN echo "rust toolchain build: $toolchainbuild"
RUN rustup install $toolchainbuild &&\
  rustup default $toolchainbuild &&\
  rustup update $toolchainbuild &&\
  rustup target add wasm32-unknown-unknown --toolchain $toolchainbuild

RUN cargo install cargo-chef
COPY --from=planner /opt/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM $RUST_IMAGE as builder
ARG toolchainbuild
WORKDIR /opt/app

RUN apt-get update && \
  apt install -y git clang curl libssl-dev llvm libudev-dev

RUN echo "rust toolchain build: $toolchainbuild"
RUN rustup install $toolchainbuild &&\
  rustup default $toolchainbuild &&\
  rustup update $toolchainbuild &&\
  rustup target add wasm32-unknown-unknown --toolchain $toolchainbuild
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /opt/app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

FROM $RUNTIME_IMAGE as runtime

EXPOSE 30333 9933 9944
COPY --from=builder /opt/app/target/release/opportunity-standalone /usr/local/bin
RUN /usr/local/bin/opportunity-standalone --version
ENTRYPOINT ["/usr/local/bin/opportunity-standalone"]