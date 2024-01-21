use std::env::args;
use std::fs::File;
use std::io::{stdin, Error, ErrorKind, Read};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum FlagParam {
    Help,
    ShowEnds,
    ShowLineNumber,
    ShowNonPrinting,
    ShowTabs,
    ShowVersion
}

#[derive(Debug, PartialEq)]
enum InputParam {
    File(String),
    Stdin
}

#[derive(Debug)]
struct CatOptions {
    inputs: Vec<InputParam>,
    flags: Vec<FlagParam>
}

struct Cat {
    opts: CatOptions
}

const TAB_CHAR: u8 = 0x9;
const LF_CHAR: u8 = 0xa;
const CR_CHAR: u8 = 0xd;

trait FileContent {
    fn add_line_number(&mut self);
    fn add_cr(&mut self);
    fn add_end_char(&mut self);
    fn add_tabs(&mut self);
    fn replace(&mut self, from: u8, to: &str);
}

impl FileContent for String {
    fn add_line_number(&mut self) {
        let mut tmp = String::new();
        let mut must_add_line_no = true;
        let mut count = 1;

        for byte in self.bytes() {
            if must_add_line_no {
                tmp.push_str(format!("{count}\t").as_str());
                must_add_line_no = false
            }

            if byte == LF_CHAR {
                must_add_line_no = true;
                count += 1
            }

            tmp.push(byte as char);
        }

        if ! tmp.is_empty() {
            self.clear();
            self.push_str(tmp.as_str())
        }
    }

    fn add_cr(&mut self) {
        self.replace(CR_CHAR, "^M")
    }

    fn add_end_char(&mut self) {
        self.replace(LF_CHAR, "$\n")
    }

    fn add_tabs(&mut self) {
        self.replace(TAB_CHAR, "^I")
    }

    fn replace(&mut self, from: u8, to: &str) {
        let mut tmp = String::new();

        for byte in self.bytes() {
            if byte == from {
                tmp.push_str(to)
            } else {
                tmp.push(byte as char)
            }
        }

        if ! tmp.is_empty() {
            self.clear();
            self.push_str(tmp.as_str())
        }
    }
}

impl CatOptions {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            flags: Vec::new()
        }
    }

    fn has_flag(&self, f: FlagParam) -> bool {
        self.flags.contains(&f)
    }

    fn add_flag(&mut self, f: FlagParam) {
        self.flags.push(f)
    }

    fn add_input(&mut self, i: InputParam) {
        self.inputs.push(i)
    }
}

impl Cat {
    fn new(opts: CatOptions) -> Self {
        Self {
            opts
        }
    }

    fn read_file(&self, filename: &String) -> Result<String, Error> {
        let mut buffer = String::new();
        let mut file = File::open(filename)?;

        // FIXME: for special files (/dev like) we stay here forever
        file.read_to_string(&mut buffer)?;

        if self.opts.has_flag(FlagParam::ShowEnds) {
            buffer.add_end_char()
        }

        if self.opts.has_flag(FlagParam::ShowTabs) {
            buffer.add_tabs()
        }

        if self.opts.has_flag(FlagParam::ShowNonPrinting) {
            buffer.add_cr()
        }

        if self.opts.has_flag(FlagParam::ShowLineNumber) {
            buffer.add_line_number()
        }

        Ok(buffer)
    }

    fn read_stdin() {
        let mut buf = String::new();

        loop {
            stdin().read_line(&mut buf).unwrap();

            print!("{buf}");
            buf.clear()
        }
    }

    fn run(&self) {
        if self.opts.has_flag(FlagParam::Help) {
            show_usage();
            return
        }

        if self.opts.has_flag(FlagParam::ShowVersion) {
            show_version();
            return
        }

        for opt in &self.opts.inputs {
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
         If FILE is not specified or be - , read the standard input.\n\n\
         \t-E, --show-ends\t\tshow $ at the end of each line\n\
         \t-n, --number\t\tnumber all output lines\n\
         \t-T, --show-tabs\t\tshow the tab chars as ^I\n\
         \t-v, --show-nonprinting\tuse the notation ^ and M-, except for LFD and TAB\n\
         \t--help\t\t\tdisplay this help and exit\n\
         \t--version\t\toutput version information and exit\n\n\
         Examples:\n\
         \tcat f - g\tEmits the content of f, after the standard input, and\n\
         \t\t\tthen the content of g at the end.\n\
         \tcat\t\tCopy the standart input to the standard output.";

    println!("{usage}")
}

fn show_version() {
    println!("cat {}", env!("CARGO_PKG_VERSION"))
}

fn parse_cli_args(args: Vec<String>) -> Result<CatOptions, Error> {
    let mut opts = CatOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-E" => opts.add_flag(FlagParam::ShowEnds),
            "-T" | "--show-tabs" => opts.add_flag(FlagParam::ShowTabs),
            "--help" => opts.add_flag(FlagParam::Help),
            "-n" | "--number" => opts.add_flag(FlagParam::ShowLineNumber),
            "-v" | "--show-nonprinting" =>
                opts.add_flag(FlagParam::ShowNonPrinting),
            "--version" => opts.add_flag(FlagParam::ShowVersion),
            "-" => opts.add_input(InputParam::Stdin),
            _ => {
                if arg.starts_with("-") {
                    let m = format!("cat: invalid option -- \"{arg}\"\n\
                                    Try cat \"--help\" for more informations.");
                    return Err(Error::new(ErrorKind::InvalidInput, m))
                } else {
                    opts.add_input(InputParam::File(arg.clone()))
                }
            }
        }
    }

    Ok(opts)
}

fn main() {
    let args = args().collect::<Vec<String>>();

    match parse_cli_args(args) {
        Ok(opts) => {
            let cat = Cat::new(opts);

            cat.run()
        },
        Err(e) => println!("{e}")
    }
}
