SHELL=/bin/bash

all: README.md

SOURCES=$(shell find -name '*.tf' -o -name '*.tfvars')

README.md-files: ${SOURCES}
	find * -name '*.tf' -o -name '*.tfvars' | sort > README.md-files
README.md: README.md-header README.md-files
	cat README.md-header > $@
	cat README.md-files | xargs ./README.md-gen.py >> $@
