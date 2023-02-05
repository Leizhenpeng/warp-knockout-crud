dev:
	cargo watch -q -c -w src/ -x run

install:
	cargo add warp
	cargo add serde --features derive
	cargo add tokio --features full
	cargo add pretty_env_logger
	cargo add uuid --features v4
	cargo add chrono --features serde
	# HotReload
	cargo install cargo-watch

deploy:
	cargo run ./target/debug/warp-knockout-crud 2>&1 | tee test.log
