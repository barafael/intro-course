# An introduction to Rust

A set of slides designed to introduce the philosophy of Rust, teach the basics, and showcase some more advanced usages and concepts.

## Prerequisites

- This repo (clone recursively to get the theme right).
- A recent Rust toolchain installed, including `rustfmt`.
- [barafael/markdown-tools](https://github.com/barafael/markdown-tools) for autolinking Rust and other items, generating snippets for slides from local Rust projects, and inserting playground buttons.
  - The repo contains a workspace with various tools. You need `linkify`, `codeblock-processor`, `snippet-extractor` (install via `cargo install --path <tool>`).
- [casey/just](https://github.com/casey/just) for running the different commands for generating and showing slides.
- [marp-team/marp](https://github.com/marp-team/marp).
- chromium (currently hardcoded in the justfile).

## HowTo display slides

- Clone this repo recursively.
- Open the workspace file with `vscode` and let it compile the Rust code.
- Run `just open $presentation_name` where `$presentation_name` is the filename without extension, such as `7_collections_and_iteration`.
- For (somewhat) continuous updates while working on the slides: `watchexec --exts md --debounce 3s "just presentation 4_expressions_and_control_flow"` (keep in mind you may have to reload in your browser for this to work).

## Issues

* Not sure this works on windows, at all... due to differences in path handling. Can't really test right now.
* Probably remove the memes
* The background imagery might need rethinking.
* The final playground in each chapter is not useful so far
* Especially the intro is all over the place
* More diagrams, illustrations