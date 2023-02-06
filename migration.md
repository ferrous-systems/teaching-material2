Command to translate from asciidoc to markdown
```
asciidoc -b docbook slides.adoc
pandoc -f docbook -t markdown_strict slides.xml -o slides.md
```
This works in a single file named `slides.adoc`, but we need it to work when recursing on directories...

```
asciidoc -b docbook slides.adoc
parent_dir_name=$(basename $(dirname $(pwd)))
pandoc -f docbook -t markdown_strict slides.xml -o $(parent_dir_name).md
```

And i could maybe use
```
find . -name "*.md" -exec sh -c "asciidoc docbook {} -s; pandoc -f docbook -t markdown_stric {}.xml -o {}.md" \;
```

this seems to have worked...
```
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

```
----------------
Next step:
I want to bring the presentations into the `mdbook` folder structure. This is problematic because all our previous presentation slides were called...`slides.md` -_-.
I now need a command to rewrite each `slides.md` into the name of if it's current working directory, which I believe I can shimmy in with 
```
parent_dir_name=$(basename $(dirname $(pwd)))
mv slides.md "$parent_dir_name".md
```
applied to every file.


