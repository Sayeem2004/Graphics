all:
	cargo build --release
	cargo run --release
	open image/*/*.ppm

test:
	cargo test --release
	rm -rf image

clean:
	cargo clean
	rm -rf image
