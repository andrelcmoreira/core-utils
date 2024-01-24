use crate::*;

#[test]
fn add_single_input_with_success() {
    let mut opts = CatOptions::new();

    opts.add_input(InputParam::Stdin);

    assert_eq!(opts.inputs.contains(&InputParam::Stdin), true);
    assert_eq!(opts.inputs.len(), 1);
}

#[test]
fn add_single_flag_with_success() {
    let mut opts = CatOptions::new();

    opts.add_flag(FlagParam::ShowTabs);

    assert_eq!(opts.flags.contains(&FlagParam::ShowTabs), true);
    assert_eq!(opts.flags.len(), 1);
}

#[test]
fn add_multiple_input_with_success() {
    let mut opts = CatOptions::new();

    opts.add_input(InputParam::File("foo".to_string()));
    opts.add_input(InputParam::File("bar".to_string()));
    opts.add_input(InputParam::File("baz".to_string()));
    opts.add_input(InputParam::File("qux".to_string()));

    assert_eq!(opts.inputs.contains(&InputParam::File("foo".to_string())), true);
    assert_eq!(opts.inputs.contains(&InputParam::File("bar".to_string())), true);
    assert_eq!(opts.inputs.contains(&InputParam::File("baz".to_string())), true);
    assert_eq!(opts.inputs.contains(&InputParam::File("qux".to_string())), true);
    assert_eq!(opts.inputs.len(), 4);
}

#[test]
fn add_multiple_flag_with_success() {
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
