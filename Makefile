ALL_CONTRACTS = csprusd csprusd-test-contract
CONTRACT_TARGET_DIR = target/wasm32-unknown-unknown/release
PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

.PHONY:	build-contract
build-contract:
	cargo build --release --target wasm32-unknown-unknown $(patsubst %, -p %, $(ALL_CONTRACTS))
	$(foreach WASM, $(ALL_CONTRACTS), wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$(WASM)).wasm ;)

setup-test: build-contract
	mkdir -p tests/wasm
	cp ./target/wasm32-unknown-unknown/release/csprusd.wasm tests/wasm
	cp ./target/wasm32-unknown-unknown/release/csprusd_test_contract.wasm tests/wasm

test: setup-test
	cd tests && cargo test

clippy:
	cd csprusd && cargo clippy --all-targets -- -D warnings
	cd csprusd-test-contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd csprusd && cargo fmt -- --check
	cd csprusd-test-contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd csprusd && cargo fmt
	cd csprusd-test-contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd csprusd && cargo clean
	cd csprusd-test-contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
