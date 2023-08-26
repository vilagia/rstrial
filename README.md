# rstrial -- A toolkit for Japanese novel writers

## What is this?

rstrial is a toolkit for Japanese novel writers. It provides a set of tools to help you write a novel in Japanese. You can convert your novel text to some formats that is suitable for publishing on the web. You can also use a text polishing advisor to improve your novel text.

### Use cases

- You want to publish your novel on a web site and print it as a book too.
  - Convert into Some formats that is suitable for publishing on the web(Aozora Bunko format).
  - Convert into Some formats that is suitable for printing as a book(Vivliostyle Flavored Markdown).
- You want to improve your novel text.

## Documented Novel Text Format on Markdown

This crates requires a specific format of novel text. The format is documented in below:

````example.md
# Title

Any text for your novel like plots or memos.

@tags tag1 tag2 tag3
```Chapter Name
Your great novel text first scene.
```

Any text for your novel like...

```Chapter Name
Your great novel text second scene.
```

````

NOTE: @tags are optional. The tags are used for text polishing advisor.

## rstrial_cli -- A Japanese novel text toolkit command line interface

### Usage

```console
$ # Required in text polishing advisor. Not required in other commands.
$ export OPENAI_API_KEY=<your api key>
$ rstrial --help
```


## rstrial_converter -- A Japanese novel text format converter library

TBW

## rstrial_parser -- A Japanese novel text lexer library

TBW
