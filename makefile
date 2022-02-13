# Making and running program
all:
	cargo build
	cargo run
	open image/ppm/*.ppm

# Running rust tests
test:
	cargo test
	rm -rf image

# Converting ppm images to png
convert:
	mkdir -p image/png
	(cd image/ppm && mogrify -path ../png -format png *.ppm)

# Cleaning up directorie
clean:
	cargo clean
	rm -rf image
