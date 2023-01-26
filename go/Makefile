SHELL=/bin/bash

all:

FOLDERS=go/ cockroachdb-errors/ protobuf/ flatbuffers/

README.md-filelist: $(shell find ${FOLDERS} -type f)
	@./README-gen-file-list.sh ${FOLDERS}
README.md:
	@cat README.md-files | xargs ./README.md-gen.py > README.md
