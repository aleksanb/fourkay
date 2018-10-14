.PHONY: all
all:
	# cargo rustc --release -- -C link-args=-static && strip target/release/fourkay && ls -la target/release
	cargo build --release && strip target/release/fourkay && ls -la target/release

.PHONY: run
run:
	cargo run --release

.PHONY: dev
dev:
	RUST_BACKTRACE=1 cargo run
