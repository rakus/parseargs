#
# Makefile for
#

# Phony targets represents recipes, not files
.PHONY: help all html man clean site

TGT_DIR := target
VERSION := $(shell cargo get version)

all: html man site

html: ${TGT_DIR}/tutorial.html ${TGT_DIR}/parseargs.html

man: ${TGT_DIR}/parseargs.1 ${TGT_DIR}/parseargs.1.gz

${TGT_DIR}/tutorial.html: tutorial.adoc
	asciidoctor -a source-highlighter=pygments -o $@ $^

${TGT_DIR}/parseargs.html: parseargs.1.adoc
	asciidoctor -a "manversion=${VERSION}" -a source-highlighter=pygments -o $@ $^

${TGT_DIR}/parseargs.1: parseargs.1.adoc
	asciidoctor -a "manversion=${VERSION}" -b manpage -o $@ $^

${TGT_DIR}/parseargs.1.gz: ${TGT_DIR}/parseargs.1
	gzip -9 -n -k ${TGT_DIR}/parseargs.1

site: html
	mkdir -p ${TGT_DIR}/site
	cp ${TGT_DIR}/tutorial.html ${TGT_DIR}/parseargs.html ${TGT_DIR}/site

clean:
	rm -rf ${TGT_DIR}






help:               ## Prints targets with help text
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%s\033[0m\n    %s\n", $$1, $$2}'
