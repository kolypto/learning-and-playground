all: README.md

SHELL := /bin/bash
FILES := $(shell cat Makefile.list)

README.md: README-head.md
	@cat README-head.md > $@
	@$(foreach f,$(FILES), \
		echo "" >> $@; \
		echo "## $(f)" >> $@; \
		echo '```$(call get_lang,$(f))' >> $@; \
		cat $(f) >> $@; \
		echo '```' >> $@; \
	)

lang.py  := python
lang.yml := yaml
get_lang = $(lang.$(subst .,,$(suffix $(1))))
