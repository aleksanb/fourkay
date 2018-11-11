.PHONY: build
build:
	xargo build --target x86_64-unknown-linux-gnu --release
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: optimize
optimize: build
	strip --strip-all -R .note* -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay
	~/upx-3.95-amd64_linux/upx -9 target/x86_64-unknown-linux-gnu/release/fourkay
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: run-optimize
run-optimize: build optimize
	./target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: run
run: build
	./target/x86_64-unknown-linux-gnu/release/fourkay
