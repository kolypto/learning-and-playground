#!/usr/bin/env bash
set -e

# Script:
# Generates the README.md-files: appends newly discovered files only.
# This is used to preserve the customized ordering of README.md-files



# Directories to search
DIRS=$@

# Output file names
OUTFILE=README.md-files
SEEN_FILES=${OUTFILE}.seen



# Files finder
# Specify your file patterns.
# Don't try to exclude everything: you can manually remove some entries anyway :)
function findfiles(){
    find $DIRS */ -type f -not \( -path "*/.*" -o -name '*.tfstate' -o -name '*.backup' \)
}


# Copy: seen files
touch $OUTFILE $SEEN_FILES
cp $SEEN_FILES $SEEN_FILES~  # make a copy: otherwise, `cat` would overwrite it
cat $OUTFILE $SEEN_FILES~ | sort | uniq  > $SEEN_FILES
rm $SEEN_FILES~

# Find new files
findfiles | { fgrep -v -x -f $SEEN_FILES || true ; } >> $OUTFILE
