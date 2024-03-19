use std::io::{stdin, Error, Read};
use std::fs::File;

use utils::filesystem::is_regular_file;

use crate::cli_param;
use crate::traits::FileContent;
use crate::{show_usage, show_version};

#[derive(Debug)]
pub struct Options {
    inputs: Vec<cli_param::Input>,
    flags: Vec<cli_param::Flag>
}

pub struct Cat {
    opts: Options
}

impl Options {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            flags: Vec::new()
        }
    }

    pub fn has_flag(&self, f: cli_param::Flag) -> bool {
        self.flags.contains(&f)
    }

    pub fn add_flag(&mut self, f: cli_param::Flag) {
        self.flags.push(f)
    }

    pub fn add_input(&mut self, i: cli_param::Input) {
        self.inputs.push(i)
    }

    pub fn inputs(&self) -> &Vec<cli_param::Input> {
        &self.inputs
    }

    pub fn flags(&self) -> &Vec<cli_param::Flag> {
        &self.flags
    }
}

impl Cat {
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
