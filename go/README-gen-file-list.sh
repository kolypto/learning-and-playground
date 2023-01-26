#!/usr/bin/env bash
set -e

DIRS=$@

OUTFILE=README.md-files
SEEN_FILES=${OUTFILE}.seen

# Generates the README.md-files: appends newly discovered files

# Copy: seen files
touch $OUTFILE $SEEN_FILES
cp $SEEN_FILES $SEEN_FILES~  # make a copy: otherwise, `cat` would overwrite it
cat $OUTFILE $SEEN_FILES~ | sort | uniq  > $SEEN_FILES
rm $SEEN_FILES~

# Find new files
find $DIRS -type f | { fgrep -v -x -f $SEEN_FILES || true ; } >> $OUTFILE
