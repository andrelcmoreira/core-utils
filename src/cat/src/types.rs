
#[derive(Debug, PartialEq)]
pub enum FlagParam {
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
pub enum InputParam {
    File(String),
    Stdin
}
