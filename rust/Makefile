SHELL=/bin/bash

all: README.md

FOLDERS=rust/ embedded/

README.md-filelist: $(shell find ${FOLDERS} -type f)
	@./README-gen-file-list.sh ${FOLDERS}
README.md: README.md-filelist
	@cat README.md-files | xargs ./README.md-gen.py > README.md
