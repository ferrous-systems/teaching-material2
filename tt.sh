#!/bin/bash
find . -name '*.adoc' > filelist.dat 
# list down all the file in a temp file
while read -r file
do
    asciidoc -b docbook "$file"
    file="${file%.adoc}"
    #get the filename witout extension
    pandoc -f docbook -t markdown_strict "$file".xml -o "$file".md

done < filelist.dat
# with while loop read each line from the file. Here each line is a locatin of .md file 

rm filelist.dat
#delete the temporary file finally
