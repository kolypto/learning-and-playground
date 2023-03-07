SHELL=/bin/bash

all: README.md

SOURCES=$(shell find */ -type f -not -path "*/.*")

README.md-files: ${SOURCES}
	./README.md-files.gen.sh */
README.md: README-header.md README.md-files
	cat README-header.md > $@
	cat README.md-files | xargs ./README.md-gen.py >> $@
