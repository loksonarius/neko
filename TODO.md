# To Do's

- Refactor error handling code
  - Maybe custom errors that include causes
  - Errors in the lib file should just trickle up to the handler func
- Add unit tests for functions
  - I might redo the handler after the error handling -- that'll give a good
    starting point to try to TDD this
  - By mocking some sample NekoApp struct for the options, I think I can add
    some testing that mocks the behavior beyond the library functions. Not sure
    about that though.
- Flesh out usage docs
  - Add full README
  - Add sample usage patterns and such in a samples directory
  - Create a series of simple bash scripts to demo functionality
- Figure out if tera reports any errors beyond what I've seen
  - There's really no good way of telling why any template may fail to render
- Use the POC version of this thing to uncover oddities
  - Dog-fooding this for some personal projects
  - Still not sure that the interface is the nicest
- Clean up project
  - Add as much info to Cargo.toml file
  - Add license and contribution guidelines
  - Set up Travis or Circle CI pipeline to build binary for multiple platforms
  - Maybe add binaries to OS distribution repos
