use std::io::{stdin, Error, Read};
use std::fs::File;

use crate::cli_param;
use crate::traits::FileContent;
use crate::{show_usage, show_version};
use utils::filesystem::is_regular_file;

#[derive(Debug)]
pub struct Options {
    inputs: Vec<cli_param::Input>,
    flags: Vec<cli_param::Flag>
}

pub struct Cat {
    opts: Options
}

impl Options {
    /// Create a new instance of the [`Options`] structure.
    ///
    /// # Return
    /// * A built [`Options`] instance.
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            flags: Vec::new()
        }
    }

    /// Check if a specific flag was supplied.
    ///
    /// See [`cli_param::Flag`] for more details.
    ///
    /// # Arguments
    /// * `flag`: The queried flag.
    ///
    /// # Return
    /// * `true` if the queried flag was supplied;
    /// * `false` if the queried flag wasn't supplied.
    pub fn has_flag(&self, flag: cli_param::Flag) -> bool {
        self.flags.contains(&flag)
    }

    /// Add a flag into the flags mapping.
    ///
    /// # Arguments
    /// * `flag`: The flag to be added.
    pub fn add_flag(&mut self, flag: cli_param::Flag) {
        self.flags.push(flag)
    }

    /// Add an input into the inputs mapping.
    ///
    /// See [`cli_param::Input`] for more details.
    ///
    /// # Arguments
    /// * `input`: The input to be added.
    pub fn add_input(&mut self, input: cli_param::Input) {
        self.inputs.push(input)
    }

    /// Get the inputs.
    ///
    /// # Return
    /// * The list of inputs.
    pub fn inputs(&self) -> &Vec<cli_param::Input> {
        &self.inputs
    }

    /// Get the flags.
    ///
    /// # Return
    /// * The list of flags.
    pub fn flags(&self) -> &Vec<cli_param::Flag> {
        &self.flags
    }
}

impl Cat {
    /// Create a new instance of the [`Cat`] structure.
    ///
    /// # Return
    /// * A built [`Cat`] instance.
    pub fn new(opts: Options) -> Self {
        Self {
            opts
        }
    }

    fn read_special_file(&self, file: &mut File) -> Result<(), Error> {
        let mut buffer = [0; 1];

        // FIXME: investigate the scenario where we don't have permission
        // enough to read the file (e.g /dev/input/event8)
        loop {
            file.read_exact(&mut buffer)?;
            print!("{}", buffer[0] as char)
        }
    }

    fn read_regular_file(&self, file: &mut File) -> Result<String, Error> {
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        if self.opts.has_flag(cli_param::Flag::SqueezeBlank) {
            buffer.squeeze_blank_lines()
        }

        if self.opts.has_flag(cli_param::Flag::ShowEnds) {
            buffer.add_end_char()
        }

        if self.opts.has_flag(cli_param::Flag::ShowTabs) {
            buffer.add_tabs()
        }

        if self.opts.has_flag(cli_param::Flag::ShowNonPrinting) {
            buffer.add_cr()
        }

        if self.opts.has_flag(cli_param::Flag::NumberNonBlank) {
            buffer.add_line_number(true)
        } else {
            if self.opts.has_flag(cli_param::Flag::ShowLineNumber) {
                buffer.add_line_number(false)
            }
        }

        Ok(buffer)
    }

    /// Read a given file.
    ///
    /// # Arguments
    /// * `filename`: Name of the file to be read.
    ///
    /// # Return
    /// * On success, the file content.
    /// * On error, the suitable [`Error`] instance.
    fn read_file(&self, filename: &String) -> Result<String, Error> {
        let mut ret = String::new();
        let mut file = File::open(filename)?;

        if is_regular_file(filename) {
            ret = self.read_regular_file(&mut file)?;
        } else {
            self.read_special_file(&mut file)?;
        }

        Ok(ret)
    }

    fn read_stdin() {
        let mut buf = String::new();

        loop {
            stdin().read_line(&mut buf)
                .unwrap();

            print!("{buf}");
            buf.clear()
        }
    }

    /// Run the application.
    pub fn run(&self) {
        if self.opts.has_flag(cli_param::Flag::ShowHelp) {
            show_usage();
            return
        }

        if self.opts.has_flag(cli_param::Flag::ShowVersion) {
            show_version();
            return
        }

        for opt in self.opts.inputs() {
            match &opt {
                cli_param::Input::Stdin => Self::read_stdin(),
                cli_param::Input::File(f) => {
                    match self.read_file(&f) {
                        Ok(buf) => print!("{buf}"),
                        Err(e) => println!("cat: {f}: {}", e)
                    }
                }
            }
        }
    }
}
