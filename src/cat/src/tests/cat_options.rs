use crate::*;

#[test]
fn add_single_input() {
    let mut opts = CatOptions::new();

    opts.add_input(InputParam::Stdin);

    assert_eq!(opts.inputs.contains(&InputParam::Stdin), true);
    assert_eq!(opts.inputs.len(), 1);
}

#[test]
fn add_single_flag() {
    let mut opts = CatOptions::new();

    opts.add_flag(FlagParam::ShowTabs);

    assert_eq!(opts.flags.contains(&FlagParam::ShowTabs), true);
    assert_eq!(opts.flags.len(), 1);
}

#[test]
fn add_multiple_inputs() {
    let mut opts = CatOptions::new();

    opts.add_input(InputParam::File("f1".to_string()));
    opts.add_input(InputParam::File("f2".to_string()));
    opts.add_input(InputParam::File("f3".to_string()));
    opts.add_input(InputParam::File("f4".to_string()));

    assert_eq!(opts.inputs.contains(&InputParam::File("f1".to_string())), true);
    assert_eq!(opts.inputs.contains(&InputParam::File("f2".to_string())), true);
    assert_eq!(opts.inputs.contains(&InputParam::File("f3".to_string())), true);
    assert_eq!(opts.inputs.contains(&InputParam::File("f4".to_string())), true);
    assert_eq!(opts.inputs.len(), 4);
}

#[test]
fn add_multiple_flags() {
    let mut opts = CatOptions::new();

    opts.add_flag(FlagParam::ShowTabs);
    opts.add_flag(FlagParam::ShowEnds);
    opts.add_flag(FlagParam::ShowNonPrinting);
    opts.add_flag(FlagParam::ShowLineNumber);

    assert_eq!(opts.flags.contains(&FlagParam::ShowTabs), true);
    assert_eq!(opts.flags.contains(&FlagParam::ShowEnds), true);
    assert_eq!(opts.flags.contains(&FlagParam::ShowNonPrinting), true);
    assert_eq!(opts.flags.contains(&FlagParam::ShowLineNumber), true);
    assert_eq!(opts.flags.len(), 4);
}

#[test]
fn has_flag_with_no_flags() {
    let opts = CatOptions::new();

    assert_eq!(opts.has_flag(FlagParam::ShowTabs), false);
    assert_eq!(opts.has_flag(FlagParam::ShowEnds), false);
    assert_eq!(opts.has_flag(FlagParam::ShowNonPrinting), false);
    assert_eq!(opts.has_flag(FlagParam::ShowLineNumber), false);
    assert_eq!(opts.has_flag(FlagParam::SqueezeBlank), false);
    assert_eq!(opts.has_flag(FlagParam::NumberNonBlank), false);
    assert_eq!(opts.has_flag(FlagParam::ShowHelp), false);
}

#[test]
fn has_flag_with_single_flag() {
    let mut opts = CatOptions::new();

    opts.add_flag(FlagParam::SqueezeBlank);

    assert_eq!(opts.has_flag(FlagParam::ShowTabs), false);
    assert_eq!(opts.has_flag(FlagParam::ShowEnds), false);
    assert_eq!(opts.has_flag(FlagParam::ShowNonPrinting), false);
    assert_eq!(opts.has_flag(FlagParam::ShowLineNumber), false);
    assert_eq!(opts.has_flag(FlagParam::SqueezeBlank), true);
    assert_eq!(opts.has_flag(FlagParam::NumberNonBlank), false);
    assert_eq!(opts.has_flag(FlagParam::ShowHelp), false);
}

#[test]
fn has_flag_with_multiple_flags() {
    let mut opts = CatOptions::new();

    opts.add_flag(FlagParam::SqueezeBlank);
    opts.add_flag(FlagParam::ShowNonPrinting);
    opts.add_flag(FlagParam::ShowLineNumber);

    assert_eq!(opts.has_flag(FlagParam::ShowTabs), false);
    assert_eq!(opts.has_flag(FlagParam::ShowEnds), false);
    assert_eq!(opts.has_flag(FlagParam::ShowNonPrinting), true);
    assert_eq!(opts.has_flag(FlagParam::ShowLineNumber), true);
    assert_eq!(opts.has_flag(FlagParam::SqueezeBlank), true);
    assert_eq!(opts.has_flag(FlagParam::NumberNonBlank), false);
    assert_eq!(opts.has_flag(FlagParam::ShowHelp), false);
}
