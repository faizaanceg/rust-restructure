#![feature(cell_extras)] 

#[macro_use(regexify)]
extern crate restructure;
extern crate regex;

use std::cell::{RefCell, Ref};
use regex::{Regex, Error};
use restructure::{Restruct, RegexStruct};

#[test]
fn single_struct() {
    regexify!(FullName {
        first, String, r"\w+" 
        ws, String, r"\s+"
        middle, String, r"\w+"
        _ws, String, r"\s+"
        last, String, r"\w+"
      });

    let name: FullName = Default::default();

    let user = Restruct::fill(&name, "Samuel Lee Jackson");

    assert_eq!("Samuel", user.first);
    assert!("L." != user.middle);
    assert_eq!("Jackson", user.last);
}
