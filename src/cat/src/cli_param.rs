#[derive(Debug, PartialEq)]
pub enum Flag {
    NumberNonBlank,
    ShowEnds,
    ShowHelp,
    ShowLineNumber,
    ShowNonPrinting,
    ShowTabs,
    ShowVersion,
    SqueezeBlank
}

#[derive(Debug, PartialEq)]
pub enum Input {
    File(String),
    Stdin
}
