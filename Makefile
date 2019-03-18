.PHONY: optimize
optimize:
	xargo build --target x86_64-unknown-linux-gnu --release --no-default-features
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay
	strip --strip-all -R .note* -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay
	~/upx-3.95-amd64_linux/upx -9 target/x86_64-unknown-linux-gnu/release/fourkay
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: run-optimize
run-optimize: optimize
	./target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: build
build:
	xargo build --target x86_64-unknown-linux-gnu --release --no-default-features

.PHONY: run
run: build
	xargo run --target x86_64-unknown-linux-gnu
