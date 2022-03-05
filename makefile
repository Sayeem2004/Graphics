# Making and running program
all:
	cargo build --release
	cargo run --release

# Running previous assignments
test:
	cargo test --release

# Converting ppm images to png
convert:
	mkdir -p image/png
	(cd image/ppm && mogrify -path ../png -format png *.ppm)

# Cleaning up directories
clean:
	cargo clean
	rm -rf image/ppm

# Removing all images
remove:
	rm -rf image

# Opening documentation
doc:
	cargo doc --open
