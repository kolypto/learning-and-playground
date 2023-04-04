SHELL=/bin/bash

all: README.md

README.md: README.md-files
	@cat README.md-files | xargs ./README.md-gen.py > README.md
