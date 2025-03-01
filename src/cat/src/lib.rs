pub mod cat;
pub mod cli_args;
pub mod traits;
pub mod cli_param;

/// Show the application usage instructions.
pub fn show_usage() {
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

/// Show the application version.
pub fn show_version() {
    println!("cat {}", env!("CARGO_PKG_VERSION"))
}
