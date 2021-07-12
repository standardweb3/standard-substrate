# Standard Substrate

![banner](./media/standard_substrate.png "Standard")

## Github Actions status

[![Tests run](https://github.com/digitalnativeinc/standard-substrate/actions/workflows/fmt-checks-tests.yml/badge.svg?branch=master)](https://github.com/digitalnativeinc/standard-substrate/actions/workflows/fmt-checks-tests.yml)
[![Build Binaries](https://github.com/digitalnativeinc/standard-substrate/actions/workflows/binary-build.yml/badge.svg?branch=master)](https://github.com/digitalnativeinc/standard-substrate/actions/workflows/binary-build.yml)

# Contact

For questions about interacting with Standard protocol, please visit [our Discord server](https://discord.standard.tech).

For security concerns, email [contact@standard.tech](mailto:contact@standard.tech).

## Overview

This repo has the implementation for Standard Protocol in Parity Substrate.

Standard protocol is the omni-stablecoin protocol secured with stability mechanisms and collaterized by digital assets across blockchains.

## Documentation

Documentation describing how Standard Protocol works is available [here](https://docs.standard.tech).

All documentation related to running a validator node can be found on [gitbook.io](https://standard-protocol-1.gitbook.io/standard-protocol-validator-guide/).

## Pre-requisites

### Rust

Installation instructions can be found by navigating to [Substrate docs](https://substrate.dev/docs/en/knowledgebase/getting-started/).

### Docker

If opting in to use Docker, you will need to install both Docker and Docker Compose - once integrated compose goes to GA separate installation of docker-compose will not be required. Up to date docs can be found here for [Docker](https://docs.docker.com/engine/install/) and for [Docker Compose](https://docs.docker.com/compose/install/).

## Using Local Standard Protocol

All commands used are referenced in a [Makefile](./Makefile).

### Initialise Rust

This step is required to work with Substrate.

```bash
make init
```

### Build project without running

```bash
make build
```

### Run project

Will build as well if not done in a previous step.

```bash
make localrun
```

## Using Docker Standard Protocol testnet

Runs docker compose which builds and runs the image.

```bash
make compose-run
```
