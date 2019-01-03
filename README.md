# neko

_Render Jinja 2 templates using a standalone binary_

This project is still heavily in-progress -- the README and documentation will
come _after_ the POC. Most of the usage is already documented in the binary
itself, try running `neko render` or `neko data' with `--help` to see the
output as below:

```
neko-render 0.1.0
Dan Herrera <sonarius@shew.io>
Renders a given Jinja 2 formatted template

The given file path will be parsed and processed after loading in all the given data files and values. This data context
will be used to evaluate variables and some expressions when rendering template files.

USAGE:
    neko render [OPTIONS] <target_template>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -D, --data <data_glob>
            Path glob to match for data input files to use when rendering

            This argument should be passed as a string, and so most shells may require wrapping this in quotes. This
            glob may be something like 'data/**/*.json', which would load all JSON object files under the 'data'
            directory and all subdirectories. It may also be a single file path like 'template-config.json' which loads
            only a single JSON object file. [default: data/**/*.json]
    -o, --output <output_file>
            Path of file where rendered output should be written to -- defaults to stdout

            If this path includes a directory without write access or to a non-existent directory, the file will fail to
            be created. If this is a path to a file that already exists, then that file will be overwritten with the
            output of this command.
    -T, --templates <templates_glob>
            Path glob to match for template input files to render

            This argument should be passed as a string, and so most shells may require wrapping this in quotes. This
            glob may be something like 'templates/**/*.j2', which would load all Jinja 2 template files under the
            'templates' directory and all subdirectories. It may also be a single file path like 'template.j2' which
            loads only a single Jinja 2 template. The reason this argument is a glob and not a single file path is that
            Jinja 2 contexts' include the concept of macros and subtemplates. This means that a template context may
            require more files than just the single source. For more info, see
            http://jinja.pocoo.org/docs/2.10/templates/#template-inheritance and
            http://jinja.pocoo.org/docs/2.10/templates/#macros. [default: templates/**/*.j2]

ARGS:
    <target_template>
            Target template to render

            This is the file name of the specific template you'd like rendered. While the 'templates' option is about
            specifying the template hierarchy files that include macros and functions and all other sorts of dependent
            templating structures, this option specifically chooses a template who's contents should be rendered. For
            example, if our templates' glob includes the 'index.html.j2' template, then we pass 'index.html.j2' as the

```

## Work to Do

- Refactor error handling code
  - Maybe custom errors that include causes
  - Errors in the lib file should just trickle up to the handler func
- Try splitting up the structopt stuff from the main function
  - Maybe I can make this a YAML-declared thing with clap alone
  - Not sure if this goes against standard Rust-eze
- Add unit tests for functions
  - I might redo the handler after the error handling -- that'll give a good
    starting point to try to TDD this
  - By mocking some sample NekoApp struct for the options, I think I can add
    some testing that mocks the behavior beyond the library functions. Not sure
    about that though.
- Flesh out usage docs
  - Add full README
  - Add sample usage patterns and such in a samples directory
- Figure out if tera reports any errors beyond what I've seen
  - There's really no good way of telling why any template may fail to render
- Implement the output flag
  - I think this can be done with a closure in the top of the handler
- Use the POC version of this thing to uncover oddities
  - Dog-fooding this for some personal projects
  - Still not sure that the interface is the nicest
- Clean up project
  - Add as much info to Cargo.toml file
  - Add license and contribution guidelines
  - Set up Travis or Circle CI pipeline to build binary for multiple platforms
  - Maybe add binaries to OS distribution repos

[:cat:](https://github.com/loksonarius/neko)
