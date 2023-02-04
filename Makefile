dev:
	cargo watch -q -c -w src/ -x run

install:
	cargo add warp
	cargo add serde --features derive
	cargo add tokio --features full
	cargo add pretty_env_logger
	cargo add uuid --features v4
	# HotReload
	cargo install cargo-watch
