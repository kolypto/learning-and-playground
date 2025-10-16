SHELL=/bin/bash

all: README.md

FOLDERS=rust/

README.md-filelist: $(shell find ${FOLDERS} -type f ! -path '**/target/**' ! -path '**/.embuild/**')
	@./README-gen-file-list.sh ${FOLDERS}
README.md: README.md-filelist
	@cat README.md-files | xargs ./README.md-gen.py > README.md
