all:
	cargo build --release
	cargo run --release
	open image1.ppm
	open image2.ppm

clean:
	cargo clean
	rm image1.ppm
	rm image2.ppm
