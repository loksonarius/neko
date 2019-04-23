# neko

[![Build Status](https://travis-ci.org/loksonarius/neko.svg?branch=master)](https://travis-ci.org/loksonarius/neko)

_Render Jinja 2 templates using a standalone binary_

## :warning: HEADS UP! :warning:
This project is still _heavily_ in flux. While I still encourage and
heavily appreciate usage this tool, I cannot currently guarantee functionality.
That said, if you find bugs and report them to me, I will personally prioritize
addressing them! :heart:

## Quick Start

Start templating with Neko today by running:

```bash
neko help
```

There's a pretty decent amount of documentation built into the tool for
reference, so please use it if you need to quickly confirm something!

## Usage

### Basic Concepts

`neko` renders templates using the Jinja2 engine. This engine has two primary
concepts to understand for usage:

| Concept | Definition |
| --- | --- |
| Context | The body of variables that can be referenced from within templates |
| Template | Document defining formatting of variables, content, and other templates|

The Context in `neko` is built by aggregating and merging all `.json` files in a
given `data` directory.

The Template in `neko` is the specific file that will be rendered after loading
all base templates, filters, and macros.

These concepts are pretty native to Jinja2, and `neko` doesn't really abstract
them much. For more info on these, please refer to
[Jinja2's docs](http://jinja.pocoo.org/).

### File Setup

`neko` has some basic expectation of default usage. Primarily, it expects a
directory structure like the one below:

```
./
├── data
│   ├── base-data.json
│   ├── sample-data.json
│   └── extra-data.json
└── templates
    ├── basic-document.txt.j2
    └── header-content-template.j2
```

The given data and templates directory will be fully traversed for `json` and
`j2` files respectively, so feel free to structure as needed for your hierarchy.

### Rendering a Template

```bash
neko render template-file-name.j2
```

The `render` command expects the full file name of the template that should be
rendered from within the 'templates' directory. The result will be printed
directly to `stdout`.

### Debugging the Context

```bash
neko data
```

It can be helpful when multiple files are involved to see what context was used
to generate templates. Instead of making a template that prints out the entire
structure of the data context, `neko` provides the convenience sub-command
`data`, which will print out the entire data context in a JSON object after
fully merging all found files.

### Development

`neko` relies mainly on the `structopt`, `serde`, and `tera` crates, and
compiles using the default Rust toolchain. Nothing terribly fancy here. There's
a Travis CI pipeline set up to compile to multiple target OS and architectures,
and that'll run for any commits to `master` and `dev`, as well as populate
releases for tagged commits to `master`.

There's currently no project management, just check the [todo's](TODO.md) for
info on what to work on, and the [contribution guide](CONTRIBUTING.md) for info
on improving `neko`!

[:cat:](https://github.com/loksonarius/neko)
