//! The `restructure` crate provides functionalities to match regexp patterns
//! into struct fields.

#![feature(cell_extras)]
extern crate regex;

use regex::{Regex, Error};
use std::cell::{RefCell, Ref};

/// This trait allows you to match struct fields with regexp
pub trait RegexStruct {
    /// This function returns Regex representation of the struct
    fn as_regex(&self) -> Ref<Regex>;
    
    /// This function takes a Slice, find Captures in it and assigns
    /// it to the appropriate struct fields
    fn find(&self, text: &str) -> Self;
}

pub struct Restruct;

impl Restruct {
    /// This function takes a `RegexStruct` and a `Slice` and returns a `RegexStruct` with its fields filled with 
    /// the patterns from the text
    ///
    ///```
    /// # #![feature(cell_extras)]
    ///
    /// # #[macro_use] extern crate restructure;
    /// # extern crate regex;
    /// # use std::cell::{RefCell, Ref};
    /// # use regex::{Regex, Error};
    /// # use restructure::{Restruct, RegexStruct};
    /// # fn main() {
    /// regexify! ( Details {
    /// name, String, r"\w+"
    /// _w, String, r"\s+"
    /// age, i32, r"\d+"
    /// });
    /// 
    ///let user: Details = Default::default();
    ///
    ///let obama = Restruct::fill(&user, "Obama 54");
    ///
    ///assert_eq!("Obama", obama.name);
    ///assert_eq!(54, obama.age);
    /// # }
    ///```
    pub fn fill<T: RegexStruct>(regex_struct: &T, text: &str) -> T {
        regex_struct.find(text)
    }
}

/// Create a struct with regex patterns and implements RegexStruct trait on it.
///
///```
/// # #![feature(cell_extras)]
/// # #[macro_use] extern crate restructure;
/// # extern crate regex;
/// # use std::cell::{RefCell, Ref};
/// # use regex::{Regex, Error};
/// # use restructure::{Restruct, RegexStruct};
/// # fn main() {
/// regexify! ( Details {
/// name, String, r"\w+"
/// _w, String, r"\s+"
/// age, i32, r"\d+"
/// });
/// # }
///```
#[macro_export]
macro_rules! regexify {
    ($name : ident {
        $($field : ident, $field_type : ty, $pattern : expr)*
    }) => {
        struct $name {
            $(
              $field: $field_type,
            )*
            _regex : RefCell<Result<Regex, Error>>
        }

        impl Default for $name {
          fn default() -> $name {
            
            let mut regex = String::new();
              
              $(
                   match stringify!($field) {
                    x if x.starts_with("_") => {
                      regex.push_str($pattern);
                    },
                    y => {
                       let capture_with_name = format!("(?P<{}>{})", y, $pattern);
                       regex.push_str(&capture_with_name);
                    }   
                   }
              )*
              
            $name {
              $($field : Default::default(),)*
              _regex : RefCell::new(Regex::new(&regex))
            } 
          }
        }
                
        impl RegexStruct for $name {

            fn as_regex(&self) -> Ref<Regex> {
              let re: Ref<Regex> = Ref::filter_map(self._regex.borrow(), |o| o.as_ref().ok()).unwrap();
              re
            }
            
            fn find(&self, text: &str) -> $name {
      
              let captures = self.as_regex().captures(text).unwrap();
      
              let mut filled : $name  = Default::default();
              
              $(
                  if let Some(value) = captures.name(stringify!($field)) {
                    filled.$field = value.parse::<$field_type>().unwrap();
                  }
              )*
              
              filled
            } 
        }
    }
}

#[cfg(test)]
mod test {

    use std::cell::{RefCell, Ref};
    use regex::{Regex, Error};
    use super::{Restruct, RegexStruct};

    #[test]
    fn single_struct_regex() {
        
        regexify!(SemVer {
        major, i32, r"\d+"
        _1, String, r"\."
        minor, i32, r"\d+"
        _2, String, r"\."
        patch, i32, r"\d+"
      });

        let version: SemVer = Default::default();

        assert_eq!(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)",
                   version.as_regex().as_str());
    }

    #[test]
    fn single_struct_with_same_types() {

        regexify!(HostName {
        domain, String, r"\w+"
        _dot, String, r"\."
        tld, String, r"\w+"
      });

        let host: HostName = Default::default();

        let filled_host = Restruct::fill(&host, "example.com");

        assert_eq!("example", filled_host.domain);
        assert_eq!("com", filled_host.tld);
    }

    #[test]
    fn single_struct_with_diff_types() {

        regexify!(Movies {
        title, String, r"'[^']+'"
        _1, String, r"\s+\("
        year, i32, r"\d+"
        _2, String, r"\)"
      });

        let movie: Movies = Default::default();

        let filled_movie = Restruct::fill(&movie, "Not my favorite movie: 'Citizen Kane' (1941).");

        assert_eq!(r"'Citizen Kane'", filled_movie.title);
        assert_eq!(1941, filled_movie.year);
    }
}
