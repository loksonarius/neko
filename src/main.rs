extern crate failure;

use failure::Error;

use self::command::run;

mod command;

fn main() -> Result<(), Error> {
    run()
}
