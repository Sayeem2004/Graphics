# Making and running program
all: install
	cargo build --release
	cargo run --release

# Running previous assignments
test: install
	cargo test --release

# Converting ppm images to png
convert:
	mkdir -p image/png
	(cd image/ppm && mogrify -path ../png -format png *.ppm)

# Cleaning up directories
clean:
	cargo clean
	rm -rf src/compiler/__pycache__
	rm -rf src/compiler/ply/__pycache__

# Removing all images
remove:
	rm -rf image

# Opening documentation
doc:
	cargo doc --open

# Installing python requirements
install:
	pip install -r data/requirements.txt
