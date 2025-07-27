SHELL := bash

# Build the binary in release mode
release:
	hack/build-release.sh

# Run cargo test
test:
	cargo test

# Generate coverage profile
coverprofile:
	hack/coverprofile.sh

# Run linter (clippy)
lint:
	cargo clippy -- --deny warnings

# Build the docs, fail on warnings
doc:
	RUSTDOCFLAGS='--deny warnings' cargo doc --no-deps

# Format the code
fmt:
	cargo fmt

# Validate that all generated files are up to date.
validate:
	hack/validate.sh

# Lint the metainfo file for Flatpak
lint-metainfo:
	flatpak run --command=flatpak-builder-lint org.flatpak.Builder appstream io.github.heathcliff26.turbo-clicker.metainfo.xml

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
	release \
	test \
	coverprofile \
	lint \
	doc \
	fmt \
	validate \
	lint-metainfo \
	clean \
	help \
	$(NULL)
