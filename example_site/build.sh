#!/bin/sh

set -xe

mkdir -p public/blog

for file in ./*.md; do # convert markdown posts to html
    static single -o "public/blog/${file%.*}.html" single.tmpl "$file"
done

# static generates the list based on the order you give it to. this means you
# have to sort the files yourself. in this case we don't really care so i just
# use glob matching.
#
# by using -i flag were also including the site.json file to be used in the
# template. Included file object will be available under the name of the file
# without the '.json' extension.
static -i "site.json" list list.tmpl ./*.md > "public/index.html"
