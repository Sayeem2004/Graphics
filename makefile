# Making and running program
all:
	cargo build
	cargo run
	open image/ppm/*.ppm

# Running rust tests
test:
	cargo test
	rm -rf image/ppm

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
