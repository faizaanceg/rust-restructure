#[macro_use(regexify)]
extern crate restructure;
extern crate regex;

use regex::Regex;
use restructure::{Restruct, RegexStruct};

#[test]
fn single_struct() {
    regexify!(FullName {
        first, String, r"\w+"
        _1, String, r"\s+"
        middle, String, r"\w+"
        _2, String, r"\s+"
        last, String, r"\w+"
      });

    let name: FullName = Default::default();
    let user = Restruct::fill(&name, "Samuel Lee Jackson");

    assert_eq!("Samuel", user.first);
    assert!("L." != user.middle);
    assert_eq!("Jackson", user.last);
}
