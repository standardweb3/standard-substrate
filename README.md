# Standard Protocol Parachain

## Github Actions status

[![Check format, setup and build](https://github.com/firke/standard-substrate/actions/workflows/main.yml/badge.svg?branch=master)](https://github.com/firke/standard-substrate/actions/workflows/main.yml)
[![Docker Build](https://github.com/firke/standard-substrate/actions/workflows/docker-build.yml/badge.svg?branch=master)](https://github.com/firke/standard-substrate/actions/workflows/docker-build.yml)

## Pre-requisites

### Supported architectures

| Architecture  | Native | Docker |
| ------------- | ------ | ------ |
| amd64         | 🟢     | 🟢     |
| windows-amd64 | 🔴     | 🔴     |
| arm64v8       | 🟡     | 🟡     |

### Rust

Installation instructions can be found locally [here](./docs/rust-install.md), or by navigating to [Substrate docs](https://substrate.dev/docs/en/knowledgebase/getting-started/).

### Docker

If opting in to use Docker, you will need to install both Docker and Docker Compose - once integrated compose goes to GA separate installation of docker-compose will not be required. Up to date docs can be found here for [Docker](https://docs.docker.com/engine/install/) and for [Docker Compose](https://docs.docker.com/compose/install/).

## Using Local Standard Protocol parachain

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

## Using Docker Standard Protocol parachain

Runs docker compose which builds and runs the image.

```bash
make compose-run
```
