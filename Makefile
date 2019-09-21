CRATE_NAME:=ksp-commnet-calculator-cli

DOC_OPTION:=--no-deps

.PHONY: all
all: build check

.PHONY: build
build: soft-clean
	cargo build

.PHONY: release-build
release-build:
	cargo build --release

.PHONY: check
check: soft-clean
	cargo test
	cargo fmt -- --check
	cargo clippy -- -D warnings

.PHONY: doc
doc:
	cargo doc $(DOC_OPTION)

.PHONY: doc-open
doc-open:
	cargo doc $(DOC_OPTION) --open

.PHONY: release
release: check release-build

.PHONY: soft-clean
soft-clean:
	- cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean
