#
# Makefile for Parseargs
#

# Phony targets represents recipes, not files
.PHONY: help debug-build release-build test script-test clean doc rpm deb prepare-archive zip tar

SRCFILES := $(wildcard src/*.rs src/**/*.rs)

TARGET_ENV := $(shell rustc -vV | sed -n 's/host: *//p')

ifneq (,$(findstring windows,$(shell echo $(OS) | tr A-Z a-z)))
	EXE_EXT := .exe
else
	EXE_EXT :=
endif

ROOT_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

VERSION := $(shell cargo get package.version)
ifndef VERSION
$(error VERSION is not set - missing 'cargo get'?)
endif

DEBUG_TGT := target/debug/parseargs${EXE_EXT}
RELEASE_TGT := target/release/parseargs${EXE_EXT}

TGT_DOC_DIR := target/doc

debug-build: ${DEBUG_TGT}                    ## Debug build using Cargo

release-build: ${RELEASE_TGT}                ## Release build using Cargo

${DEBUG_TGT}: Cargo.toml ${SRCFILES}
	cargo build

${RELEASE_TGT}: Cargo.toml ${SRCFILES}
	cargo build --release

unit-test:                                   ## run Cargo tests
	cargo test

script-test:                                 ## run shell script tests
	./script-test/run.sh

test: unit-test script-test                  ## run Cargo and shell script tests

check: clean debug-build test                ## run clean debug build, tests, format check etc
	cargo fmt --check
	( cd script-test && shellcheck -fgcc -x -a *.sh )

doc: html-doc manpage                        ## build documentation (tutorial, man page)

html-doc: ${TGT_DOC_DIR}/tutorial.html ${TGT_DOC_DIR}/parseargs.html    # build HTML documentation

manpage: ${TGT_DOC_DIR}/parseargs.1 ${TGT_DOC_DIR}/parseargs.1.gz       # build man page

${TGT_DOC_DIR}/tutorial.html: doc/tutorial.adoc
	asciidoctor -a "version=${VERSION}" -a source-highlighter=pygments -o $@ $^

${TGT_DOC_DIR}/parseargs.html: doc/parseargs.1.adoc
	asciidoctor -a "version=${VERSION}" -a source-highlighter=pygments -o $@ $^

${TGT_DOC_DIR}/parseargs.1: doc/parseargs.1.adoc
	asciidoctor -a "version=${VERSION}" -b manpage -o $@ $^

${TGT_DOC_DIR}/parseargs.1.gz: ${TGT_DOC_DIR}/parseargs.1
	gzip -f -9 -n -k $^

rpm: release-build doc                       ## Build rpm package
	strip -s target/release/parseargs
	cargo generate-rpm

deb: release-build doc                       ## Build deb package
	strip -s target/release/parseargs
	cargo deb

pkg: rpm deb                                 ## Build rpm & deb packages

zip: target/parseargs-${VERSION}-${TARGET_ENV}.zip      ## build zip of release build (incl man page as html)

tar: target/parseargs-${VERSION}-${TARGET_ENV}.tar.gz   ## build tar.gz of release build (incl man page as html)

prepare-archive: release-build doc
	rm -rf target/archive
	mkdir -p  target/archive
	cp target/release/parseargs${EXE_EXT} target/doc/parseargs.html target/archive


target/parseargs-${VERSION}-${TARGET_ENV}.zip: prepare-archive
	rm -f $@
	zip -j --must-match $@ target/archive/*

target/parseargs-${VERSION}-${TARGET_ENV}.tar.gz: prepare-archive
	rm -f $@
	(cd target/archive && tar -czvf ${ROOT_DIR}$@ *)

clean:
	cargo clean

help:                                        ## Prints targets with help text
	@cat ${MAKEFILE_LIST} | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%s\033[0m\n    %s\n", $$1, $$2}'
