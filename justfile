snippets:
    snippet-extractor --directory projects --output snippets.json --relative

autolink:
    markdown-linkify language-basics-01.md --config linkify.toml --output language-basics-01-autolinked.md

codeblocks:
    markdown-codeblock-processor language-basics-01-autolinked.md --snippets snippets.json -o language-basics-01-final.md

marp:
    marp --allow-local-files --html --pdf-outlines.headings true --pdf-outlines.pages true language-basics-01-final.md --html true --theme marp-theme-rhea/rhea.css

presentation: snippets autolink codeblocks marp

watch: presentation
    chromium language-basics-01-final.html
    watchexec --exts md --debounce 10s "just presentation"

all: presentation
    chromium language-basics-01-final.html
