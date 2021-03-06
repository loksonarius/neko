extern crate failure;
extern crate structopt;

use std::{fs::File, io::prelude::*, path::PathBuf};
use failure::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "neko", about = "Renders Jinja 2 formatted templates")]
pub struct NekoApp {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    /// Verbosity to use when printing output to stdout
    ///
    /// The more times this flag is used the more verbose the output becomes. The max verbosity is
    /// 3, and values higher than this will be ignored. If printing to a file using the --output/-o
    /// flags, the verbose output will not be included and only be printed to stdout.
    verbosity: u64,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
struct DataLoadingOpts {
    // I await the day someone complains about this argument being singular while the template one
    // is plural just so I can explain that 'data' is indeed plural while 'datum' is singular :^3
    #[structopt(short = "D", long = "data", default_value = "data/**/*.json")]
    /// Path glob to match for data input files to use when rendering
    ///
    /// This argument should be passed as a string, and so most shells may require wrapping this in
    /// quotes. This glob may be something like 'data/**/*.json', which would load all JSON object
    /// files under the 'data' directory and all subdirectories. It may also be a single file path
    /// like 'template-config.json' which loads only a single JSON object file.
    data_glob: String,
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    /// Path of file where rendered output should be written to -- defaults to stdout
    ///
    /// If this path includes a directory without write access or to a non-existent directory, the
    /// file will fail to be created. If this is a path to a file that already exists, then that
    /// file will be overwritten with the output of this command.
    output_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "data")]
    /// Prints out the aggregated data struct that's used when rendering templates
    Data {
        #[structopt(flatten)]
        data_loading_opts: DataLoadingOpts,
    },
    #[structopt(name = "render")]
    /// Renders a given Jinja 2 formatted template
    ///
    /// The given file path will be parsed and processed after loading in all the given data files
    /// and values. This data context will be used to evaluate variables and some expressions when
    /// rendering template files.
    Render {
        #[structopt(flatten)]
        data_loading_opts: DataLoadingOpts,
        #[structopt(short = "T", long = "templates", default_value = "templates/**/*.j2")]
        /// Path glob to match for template input files to render
        ///
        /// This argument should be passed as a string, and so most shells may require wrapping
        /// this in quotes. This glob may be something like 'templates/**/*.j2', which would load
        /// all Jinja 2 template files under the 'templates' directory and all subdirectories. It
        /// may also be a single file path like 'template.j2' which loads only a single Jinja 2
        /// template. The reason this argument is a glob and not a single file path is that Jinja 2
        /// contexts' include the concept of macros and subtemplates. This means that a template
        /// context may require more files than just the single source. For more info, see
        /// http://jinja.pocoo.org/docs/2.10/templates/#template-inheritance and
        /// http://jinja.pocoo.org/docs/2.10/templates/#macros.
        templates_glob: String,
        /// Target template to render
        ///
        /// This is the file name of the specific template you'd like rendered. While the
        /// 'templates' option is about specifying the template hierarchy files that include macros
        /// and functions and all other sorts of dependent templating structures, this option
        /// specifically chooses a template who's contents should be rendered. For example, if our
        /// templates' glob includes the 'index.html.j2' template, then we pass 'index.html.j2' as
        /// the value for this parameter to get the rendered result.
        target_template: String,
        },
}

fn print_results(output: String, target: Option<PathBuf>) -> Result<(), Error> {
        match target {
            Some(path) => {
                let mut file = File::create(path)?;
                file.write_all(output.as_bytes())?;
                info!("Printed results to output file");
            },
            None => {
                println!("{}", output);
            },
        }
        Ok(())
}

fn handle(opts: NekoApp) -> Result<(), Error> {
    loggerv::init_with_verbosity(opts.verbosity)?;
    match opts.cmd {
        Command::Data{
            data_loading_opts: DataLoadingOpts{
                output_file, data_glob
            }} => {
                let data = neko::parse_data_glob(&data_glob)?;
                let output = serde_json::to_string_pretty(&data)?;
                print_results(output, output_file)?;
        },
        Command::Render{
            templates_glob,
            target_template,
            data_loading_opts: DataLoadingOpts{
                output_file, data_glob
            }} => {
                let data = neko::parse_data_glob(&data_glob)?;
                let templates = neko::compile_templates(&templates_glob)?;
                let output = neko::render_template(templates, data, &target_template)?;
                print_results(output, output_file)?;
        },
    }
    Ok(())
}

pub fn run() -> Result<(), Error> {
    handle(NekoApp::from_args())
}
