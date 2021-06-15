#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup install nightly-2021-03-01
   rustup default nightly-2021-03-01
   rustup update nightly-2021-03-01
   rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-01
fi

rustup target add wasm32-unknown-unknown --toolchain nightly
