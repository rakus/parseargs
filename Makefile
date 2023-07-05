#
# Makefile for
#

# Phony targets represents recipes, not files
.PHONY: help debug-build release-build test script-test clean doc rpm deb

DEBUG_TGT := target/debug/parseargs
RELEASE_TGT := target/release/parseargs

SRCFILES := $(wildcard src/*.rs src/**/*.rs)

VERSION := $(shell cargo get version)


debug-build: ${DEBUG_TGT}                    ## Debug build the application using cargo

release-build: ${RELEASE_TGT}                ## Release build the application using cargo

${DEBUG_TGT}: Cargo.toml ${SRCFILES}
	cargo build

${RELEASE_TGT}: Cargo.toml ${SRCFILES}
	cargo build --release

unit-test:                                   ## run Cargo tests
	cargo test

script-test:                                 ## run shell script tests
	./script-test/run.sh

test: unit-test script-test                     ## run unit and shell script tests

check: clean debug-build test                ## run clean debug build and test
	( cd script-test && shellcheck -fgcc -x -a *.sh )

doc:
	( cd doc && make VERSION=$(VERSION) )

rpm: release-build doc                       ## Build rpm package
	#cargo install cargo-generate-rpm
	strip -s target/release/parseargs
	cargo generate-rpm

deb: release-build doc                       ## Build deb package
	#cargo install cargo-generate-rpm
	strip -s target/release/parseargs
	cargo deb

pkg: rpm deb                                 ## Build rpm & deb packages

setup:                                       ## Install needed cargo commands
	cargo install cargo-get
	cargo install cargo-generate-rpm
	cargo install cargo-deb

clean:
	cargo clean
	( cd doc && make clean )

help:                                        ## Prints targets with help text
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%s\033[0m\n    %s\n", $$1, $$2}'
