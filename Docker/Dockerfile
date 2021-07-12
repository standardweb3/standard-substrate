ARG RUST_IMAGE=rust:1.52-slim
ARG RUNTIME_IMAGE=ubuntu:focal
ARG TOOLCHAIN_BUILD=nightly-2021-04-22

### planner stage - builds dependency recipe.json for cargo chef
FROM $RUST_IMAGE as planner
ARG TOOLCHAIN_BUILD
WORKDIR /opt/app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN echo "rust toolchain build: $TOOLCHAIN_BUILD"
RUN rustup default $TOOLCHAIN_BUILD
RUN cargo install cargo-chef 

COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

### caching stage - builds out dependency cache
FROM $RUST_IMAGE as cacher
ARG TOOLCHAIN_BUILD
WORKDIR /opt/app

RUN apt-get update && \
  apt install -y git clang curl libssl-dev llvm libudev-dev
RUN echo "rust toolchain build: $TOOLCHAIN_BUILD"
RUN rustup install $TOOLCHAIN_BUILD &&\
  rustup default $TOOLCHAIN_BUILD &&\
  rustup update $TOOLCHAIN_BUILD &&\
  rustup target add wasm32-unknown-unknown --toolchain $TOOLCHAIN_BUILD

RUN cargo install cargo-chef
COPY --from=planner /opt/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

### building stage - copies dependencies cache, builds the application
FROM $RUST_IMAGE as builder
ARG TOOLCHAIN_BUILD
WORKDIR /opt/app

RUN apt-get update && \
  apt install -y git clang curl libssl-dev llvm libudev-dev

RUN echo "rust toolchain build: $TOOLCHAIN_BUILD"
RUN rustup install $TOOLCHAIN_BUILD &&\
  rustup default $TOOLCHAIN_BUILD &&\
  rustup update $TOOLCHAIN_BUILD &&\
  rustup target add wasm32-unknown-unknown --toolchain $TOOLCHAIN_BUILD

COPY . .
# Copy over the cached dependencies
COPY --from=cacher /opt/app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

RUN cargo build --release

### runtime stage - pulls a small image and pushes the application
FROM $RUNTIME_IMAGE as runtime

LABEL maintainer="Standard Tech <tech@standard.tech>"
LABEL description="Standard Tech Opportunity Node"
EXPOSE 30333 9933 9944
COPY --from=builder --chmod=u=rwx,go=rx /opt/app/target/release/opportunity-standalone /usr/local/bin
COPY ./Docker/entrypoint.sh /
ENTRYPOINT ["/entrypoint.sh", "/usr/local/bin/opportunity-standalone"]
CMD ["--help"]