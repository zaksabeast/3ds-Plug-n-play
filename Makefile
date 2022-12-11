.PHONY: clean test lint release

all: test lint release

clean:
	@cargo clean
	@make clean -C pnp_sys
	@make clean -C launcher
	@rm -rf out

test:
	@cargo +nightly test --features test_stubs

lint:
	@make lint -C pnp_sys
	@cargo +nightly clippy --manifest-path example_plugin/Cargo.toml
	@cargo +nightly clippy --manifest-path pnp_lib/Cargo.toml

release:
	@make release -C pnp_sys
	@touch pnp_sys/src/main.rs && MODE3=1 make release -C pnp_sys
	@make -C launcher
	@cargo build --release --manifest-path example_plugin/Cargo.toml --target wasm32-unknown-unknown
	@mkdir -p out/pnp
	@cp pnp_sys/out/release/pnp_sys.cia out/.
	@cp pnp_sys/out/release/pnp_sys_mode3.cia out/.
	@cp launcher/out/pnp_launcher.cia out/.
	@cp target/wasm32-unknown-unknown/release/example_plugin.wasm out/pnp/.
