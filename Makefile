#
# Makefile for Parseargs
#

# Phony targets represents recipes, not files
.PHONY: help debug-build release-build test script-test clean doc rpm deb prepare-archive zip tar

SRCFILES := $(wildcard src/*.rs src/**/*.rs)

OS_NAME := $(shell uname -s | tr A-Z a-z)
PROC_NAME := $(shell uname -m)

BUILD_ENV := ${PROC_NAME}-${OS_NAME}

ifneq (,$(findstring windows,$(shell echo $(OS) | tr A-Z a-z)))
	EXE_EXT := .exe
else
	EXE_EXT :=
endif

ROOT_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

VERSION := $(shell cargo get version)
ifndef VERSION
$(error VERSION is not set - missing 'cargo get'?)
endif

DEBUG_TGT := target/debug/parseargs${EXE_EXT}
RELEASE_TGT := target/release/parseargs${EXE_EXT}


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

check: clean debug-build test                ## run clean debug build, format check etc
	cargo fmt --check
	( cd script-test && shellcheck -fgcc -x -a *.sh )

doc:
	( cd doc && make VERSION=${VERSION} )

rpm: release-build doc                       ## Build rpm package
	strip -s target/release/parseargs
	cargo generate-rpm

deb: release-build doc                       ## Build deb package
	strip -s target/release/parseargs
	cargo deb

pkg: rpm deb                                 ## Build rpm & deb packages

zip: target/parseargs-${VERSION}-${BUILD_ENV}.zip      # build zip of release build (incl man page as html)

tar: target/parseargs-${VERSION}-${BUILD_ENV}.tar.gz   # build tar.gz of release build (incl man page as html)

prepare-archive: release-build doc
	rm -rf target/archive
	mkdir -p  target/archive
	cp target/release/parseargs${EXE_EXT} doc/target/parseargs.html target/archive


target/parseargs-${VERSION}-${BUILD_ENV}.zip: prepare-archive
	rm -f $@
	zip -j --must-match $@ target/archive/*

target/parseargs-${VERSION}-${BUILD_ENV}.tar.gz: prepare-archive
	rm -f $@
	(cd target/archive && tar -czvf ${ROOT_DIR}$@ *)

setup:                                       ## Install needed cargo commands
	cargo install cargo-get
	cargo install cargo-generate-rpm
	cargo install cargo-deb

clean:
	cargo clean
	( cd doc && make clean )

help:                                        ## Prints targets with help text
	@cat ${MAKEFILE_LIST} | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%s\033[0m\n    %s\n", $$1, $$2}'
