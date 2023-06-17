#
# Makefile for
#

# Phony targets represents recipes, not files
.PHONY: help

TGT_DIR=target


${TGT_DIR}/tutorial.html: tutorial.adoc
	asciidoctor -a source-highlighter=pygments -o $@ $^


help:               ## Prints targets with help text
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%s\033[0m\n    %s\n", $$1, $$2}'
