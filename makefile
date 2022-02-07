all:
	cargo build --release
	cargo run --release
	open *.ppm

clean:
	cargo clean
	rm *.ppm
