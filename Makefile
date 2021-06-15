.PHONY: init
init:
	./scripts/init.sh

.PHONY: format
format:
	SKIP_WASM_BUILD=1 cargo fmt --all

.PHONY: ci-format
ci-format:
	SKIP_WASM_BUILD=1 cargo fmt --all -- --check

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all

.PHONY: build
build:
	cargo build --release

.PHONY: build-opportunity-standalone
build-opportunity-standalone:
	cargo build --release --bin opportunity-standalone

# Only test business logics without applying standalone consensus
.PHONY: localrun
localrun:
	cargo run --bin=opportunity-standalone --release -- --dev --tmp

# Run polkadot parachain local testnet setup
.PHONY: polkarun
polkarun:
	cargo build --release; ./polkadot/target/release/polkadot build-spec --chain rococo-local --raw --disable-default-bootnode > rococo_local.json; ./polkadot/target/release/polkadot --chain ./rococo_local.json -d cumulus_relay0 --validator --alice --port 50556 & sleep 10; ./polkadot/target/release/polkadot --chain ./rococo_local.json -d cumulus_relay1 --validator --bob --port 50555 & sleep 10; ./target/release/standard-collator -d local-test --collator --alice --ws-port 9945 --parachain-id 200 -- --chain ./rococo_local.json;

# Run bob standalone node
.PHONY: run1
run1:
	./target/release/opportunity-standalone -d local-test1 --alice --ws-port 9950;

# Run alice standalone node
.PHONY: run2
run2:
	./target/release/opportunity-standalone -d local-test2 --bob --ws-port 9951;

.PHONY: run-collator1
run-collator1:
	./target/release/standard-collator -d local-bob --alice --ws-port 9950;

.PHONY: run-collator2
run-collator2:
	./target/release/standard-collator -d local-alice --bob --ws-port 9947 --rpc-port 9951;

.PHONY: compose-run
compose-run:
	docker-compose up -d

.PHONY: docker-build
docker-build:
	DOCKER_BUILDKIT=1 docker build -f Docker/Dockerfile -t standard-opportunity:local .

.PHONY: docker-run
docker-run:
	docker run --rm -it standard-opportunity:local