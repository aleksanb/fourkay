TARGET=i686-unknown-linux-gnu
TARGET=x86_64-unknown-linux-gnu

.PHONY: default-shaders
default-shaders:
	ls src/shaders/2022/*.glsl | xargs -i{} cp {} {}.out

.PHONY: compile-shaders
compile-shaders:
	ls src/shaders/2022/*.glsl | xargs -i{} mono shader_minifier.exe --format text -o {}.out --aggressive-inlining --preserve-externals {}
	ls src/shaders/2022/*.glsl.out | xargs wc --bytes

.PHONY: debug
debug:
	cargo run --features println --features error-handling

.PHONY: optimize-build
optimize-build:
	cargo build  --release
	wc --bytes target/$(TARGET)/release/fourkay

.PHONY: pack
pack: optimize-build
	rm -rf build && mkdir build

	strip --strip-all -R '.note*' -R .comment target/$(TARGET)/release/fourkay
	wc --bytes target/$(TARGET)/release/fourkay

	nasm -fbin -obuild/vondehi ../vondehi/vondehi.asm
	lzma -c target/$(TARGET)/release/fourkay > build/fourkay-lzma
	cat build/vondehi build/fourkay-lzma > build/fourkay

	chmod +x build/fourkay
	wc --bytes build/fourkay

.PHONY: run-optimize
run: pack
	build/fourkay

# target/${TARGET}/release/fourkay


#.PHONY: optimize
#optimize: debug-run
#	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay
#	strip --strip-all -R .note* -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
#	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay