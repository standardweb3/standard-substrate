#!/usr/bin/env bash

set -e

echo "*** Check if root"

if [[ $(whoami) == "root" ]]; then
    MAKE_ME_ROOT=
    echo "root already, continuing"
else
    MAKE_ME_ROOT=sudo
    echo "will use sudo"
fi

echo "*** Setting up dependencies"

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	if [ -f /etc/SuSE-release ]; then
		echo "Suse Linux detected."
		$MAKE_ME_ROOT zypper install clang curl git openssl-devel llvm-devel libudev-devel
	elif [ -f /etc/arch-release ]; then
		echo "Arch Linux detected."
      pacman -Syu --needed --noconfirm curl git clang
	elif [ -f /etc/debian_version ]; then
		echo "Ubuntu/Debian Linux detected."
      $MAKE_ME_ROOT apt update
      $MAKE_ME_ROOT apt install -y git clang curl libssl-dev llvm libudev-dev
	else
		echo "Unknown Linux distribution - no dependencies specified."
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
   echo "MacOS detected."
   brew update
   brew install openssl
else
	echo "Unknown operating system - no dependencies specified."
fi

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup default stable
   rustup install nightly-2021-09-12
   rustup target add wasm32-unknown-unknown --toolchain nightly-2021-09-12
fi
