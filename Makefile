.PHONY: host remote

export RUST_LOG=info

host:
	cargo run 4000

remote:
	cargo run 4001 /ip4/127.0.0.1/tcp/4000