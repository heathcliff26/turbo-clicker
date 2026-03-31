SHELL := bash

# Build the binary for testing
build: install-deps
	cargo build

# Build the binary in release mode and create release bundle
release: install-deps
	hack/build-release.sh

# Run cargo test
test: install-deps
	cargo test

# Generate coverage profile
coverprofile: install-deps
	hack/coverprofile.sh

# Run linter (clippy)
lint: install-deps
	cargo clippy -- --deny warnings

# Build the docs, fail on warnings
doc: install-deps
	RUSTDOCFLAGS='--deny warnings' cargo doc --no-deps

# Format the code
fmt:
	cargo fmt

# Validate that all generated files are up to date.
validate:
	hack/validate.sh

# Validate the appstream metainfo file
validate-metainfo:
	appstreamcli validate io.github.heathcliff26.turbo-clicker.metainfo.xml

# Install all dependencies needed for development
install-deps:
	hack/install-deps.sh

# Clean up generated files
clean:
	hack/clean.sh

# Show this help message
help:
	@echo "Available targets:"
	@echo ""
	@awk '/^#/{c=substr($$0,3);next}c&&/^[[:alpha:]][[:alnum:]_-]+:/{print substr($$1,1,index($$1,":")),c}1{c=0}' $(MAKEFILE_LIST) | column -s: -t
	@echo ""
	@echo "Run 'make <target>' to execute a specific target."

.PHONY: \
	build \
	release \
	test \
	coverprofile \
	lint \
	doc \
	fmt \
	validate \
	validate-metainfo \
	install-deps \
	clean \
	help \
	$(NULL)
