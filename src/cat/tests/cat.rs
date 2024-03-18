use cat::it::Options;
use cat::cli_param::{Flag, Input};

#[test]
fn add_single_input() {
    let mut opts = Options::new();

    opts.add_input(Input::Stdin);

    assert_eq!(opts.inputs().contains(&Input::Stdin), true);
    assert_eq!(opts.inputs().len(), 1);
}

#[test]
fn add_single_flag() {
    let mut opts = Options::new();

    opts.add_flag(Flag::ShowTabs);

    assert_eq!(opts.flags().contains(&Flag::ShowTabs), true);
    assert_eq!(opts.flags().len(), 1);
}

#[test]
fn add_multiple_inputs() {
    let mut opts = Options::new();

    opts.add_input(Input::File("f1".to_string()));
    opts.add_input(Input::File("f2".to_string()));
    opts.add_input(Input::File("f3".to_string()));
    opts.add_input(Input::File("f4".to_string()));

    assert_eq!(opts.inputs().contains(&Input::File("f1".to_string())), true);
    assert_eq!(opts.inputs().contains(&Input::File("f2".to_string())), true);
    assert_eq!(opts.inputs().contains(&Input::File("f3".to_string())), true);
    assert_eq!(opts.inputs().contains(&Input::File("f4".to_string())), true);
    assert_eq!(opts.inputs().len(), 4);
}

#[test]
fn add_multiple_flags() {
    let mut opts = Options::new();

    opts.add_flag(Flag::ShowTabs);
    opts.add_flag(Flag::ShowEnds);
    opts.add_flag(Flag::ShowNonPrinting);
    opts.add_flag(Flag::ShowLineNumber);

    assert_eq!(opts.flags().contains(&Flag::ShowTabs), true);
    assert_eq!(opts.flags().contains(&Flag::ShowEnds), true);
    assert_eq!(opts.flags().contains(&Flag::ShowNonPrinting), true);
    assert_eq!(opts.flags().contains(&Flag::ShowLineNumber), true);
    assert_eq!(opts.flags().len(), 4);
}

#[test]
fn has_flag_with_no_flags() {
    let opts = Options::new();

    assert_eq!(opts.has_flag(Flag::ShowTabs), false);
    assert_eq!(opts.has_flag(Flag::ShowEnds), false);
    assert_eq!(opts.has_flag(Flag::ShowNonPrinting), false);
    assert_eq!(opts.has_flag(Flag::ShowLineNumber), false);
    assert_eq!(opts.has_flag(Flag::SqueezeBlank), false);
    assert_eq!(opts.has_flag(Flag::NumberNonBlank), false);
    assert_eq!(opts.has_flag(Flag::ShowHelp), false);
}

#[test]
fn has_flag_with_single_flag() {
    let mut opts = Options::new();

    opts.add_flag(Flag::SqueezeBlank);

    assert_eq!(opts.has_flag(Flag::ShowTabs), false);
    assert_eq!(opts.has_flag(Flag::ShowEnds), false);
    assert_eq!(opts.has_flag(Flag::ShowNonPrinting), false);
    assert_eq!(opts.has_flag(Flag::ShowLineNumber), false);
    assert_eq!(opts.has_flag(Flag::SqueezeBlank), true);
    assert_eq!(opts.has_flag(Flag::NumberNonBlank), false);
    assert_eq!(opts.has_flag(Flag::ShowHelp), false);
}

#[test]
fn has_flag_with_multiple_flags() {
    let mut opts = Options::new();

    opts.add_flag(Flag::SqueezeBlank);
    opts.add_flag(Flag::ShowNonPrinting);
    opts.add_flag(Flag::ShowLineNumber);

    assert_eq!(opts.has_flag(Flag::ShowTabs), false);
    assert_eq!(opts.has_flag(Flag::ShowEnds), false);
    assert_eq!(opts.has_flag(Flag::ShowNonPrinting), true);
    assert_eq!(opts.has_flag(Flag::ShowLineNumber), true);
    assert_eq!(opts.has_flag(Flag::SqueezeBlank), true);
    assert_eq!(opts.has_flag(Flag::NumberNonBlank), false);
    assert_eq!(opts.has_flag(Flag::ShowHelp), false);
}

//#[test]
//fn read_file_with_no_opts() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_squeeze_blank() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_show_ends() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_add_tabs() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_add_cr() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_number_non_blank() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_show_line_number() {
//    // TODO
//}
//
//#[test]
//fn read_file_with_show_line_number_and_number_non_blank() {
//    // TODO
//}
