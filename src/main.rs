extern crate failure;
#[macro_use]
extern crate log;
extern crate loggerv;
extern crate structopt;
extern crate tera;

use failure::Error;

mod command;

fn main() -> Result<(), Error> {
    command::run()
}
