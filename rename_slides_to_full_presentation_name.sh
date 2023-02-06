#!/bin/bash

#parent_dir_name=$(basename "$(dirname "$(pwd)")")

#cp slides.md "$parent_dir_name".md
# Bail out if not in src/`
cd src/presentations/  || return -1

# Iterate over directory names
for d in */; do
    # check that a `slides.md` exists, else bail 

    # parent_dir_name exists, get it without the slash
    currdir=$(basename "$d")
    dname=$(dirname "$d")
    echo "$currdir"
    echo "$dname"
    echo ""
done

cd ../..


