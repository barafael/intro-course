snippets:
    snippet-extractor --directory snippets --output snippets.json --relative

autolink target:
    markdown-linkify {{target}}.md --config linkify.toml --output {{target}}.autolinked

codeblocks target:
    markdown-codeblock-processor {{target}}.autolinked --snippets snippets.json -o {{target}}.final.md

pdf target:
    marp --allow-local-files --pdf --pdf-outlines.headings true --pdf-outlines.pages true {{target}}.final.md --html true --theme marp-theme-rhea/rhea.css

html target:
    marp --allow-local-files --html --pdf-outlines.headings true --pdf-outlines.pages true {{target}}.final.md --html true --theme marp-theme-rhea/rhea.css

presentation target: snippets (autolink target) (codeblocks target) (html target) (pdf target)

open target: (presentation target)
    chromium {{target}}.final.html &
