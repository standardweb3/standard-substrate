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

.PHONY: build-standard-collator
build-standard-collator:
	cargo build --release --bin standard-collator

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

.PHONY: docker-build
docker-build:
	DOCKER_BUILDKIT=1 docker build -f docker/Dockerfile -t opportunity-standalone:local .

# example: make docker-run VOLUME_PATH='./data' DATA_DIR='/data' NODE_NAME='Standard Validator' 
.PHONY: docker-run
docker-run:
	docker run --rm -it -v "$(VOLUME_PATH)":"$(DATA_DIR)" \
		--name "${NODE}" \
		standardprotocol/"${NODE}":latest \
		--base-path "$(DATA_DIR)" \
		--chain opportunity \
		--port 30333 \
		--name "$(NAME)" \
		--validator

# example: make docker-compose-run NODE='opportunity-standalone' CHAIN='opportunity'
.PHONY: docker-compose-run
docker-compose-run:
	NODE="$(NODE)" CHAIN="${CHAIN}" docker-compose -f ./docker/docker-compose.yml up --detach

.PHONY: docker-logs
docker-logs:
	docker logs standard-validator -f