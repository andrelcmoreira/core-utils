use std::io::{stdin, Error, Read};
use std::fs::File;

use crate::cat_options::CatOptions;
use crate::types::{FlagParam, InputParam};
use crate::file_content::FileContent;
use utils::filesystem::is_regular_file;

pub struct Cat {
    opts: CatOptions
}

impl Cat {
    pub fn new(opts: CatOptions) -> Self {
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

        if self.opts.has_flag(FlagParam::SqueezeBlank) {
            buffer.squeeze_blank_lines()
        }

        if self.opts.has_flag(FlagParam::ShowEnds) {
            buffer.add_end_char()
        }

        if self.opts.has_flag(FlagParam::ShowTabs) {
            buffer.add_tabs()
        }

        if self.opts.has_flag(FlagParam::ShowNonPrinting) {
            buffer.add_cr()
        }

        if self.opts.has_flag(FlagParam::NumberNonBlank) {
            buffer.add_line_number(true)
        } else {
            if self.opts.has_flag(FlagParam::ShowLineNumber) {
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
        if self.opts.has_flag(FlagParam::ShowHelp) {
            show_usage();
            return
        }

        if self.opts.has_flag(FlagParam::ShowVersion) {
            show_version();
            return
        }

        for opt in self.opts.inputs() {
            match &opt {
                InputParam::Stdin => Self::read_stdin(),
                InputParam::File(f) => {
                    match self.read_file(&f) {
                        Ok(buf) => print!("{buf}"),
                        Err(e) => println!("cat: {f}: {}", e)
                    }
                }
            }
        }
    }
}

fn show_usage() {
    let usage =
        "Usage: cat [OPTION]... [FILE]...\n\
         Concatenate FILE(S) to the standard output.\n\n\
         If FILE is not specified or be -, read the standard input.\n\n\
         \t-A, --show-all\t\tequivalent to -vET\n\
         \t-b, --number-nonblank\tnumber non blank output lines, overlaps -n\n\
         \t-e\t\t\tequivalent to -vE\n\
         \t-E, --show-ends\t\tshow $ at the end of each line\n\
         \t-n, --number\t\tnumber all output lines\n\
         \t-s, --squeeze-blank\tsuppress repeated blank lines\n\
         \t-t\t\t\tequivalent to -vT\n\
         \t-T, --show-tabs\t\tshow the tab chars as ^I\n\
         \t-u\t\t\t(ignored)\n\
         \t-v, --show-nonprinting\tuse the notation ^ and M-, except for LFD and TAB\n\
         \t--help\t\t\tdisplay this help and exit\n\
         \t--version\t\toutput version information and exit\n\n\
         Examples:\n\
         \tcat f - g\tEmits the content of f, after the standard input, and\n\
         \t\t\tthen the content of g at the end.\n\
         \tcat\t\tCopy the standard input to the standard output.";

    println!("{usage}")
}

fn show_version() {
    println!("cat {}", env!("CARGO_PKG_VERSION"))
}
