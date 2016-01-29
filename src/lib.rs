#![feature(cell_extras)]
extern crate regex;

use regex::{Regex, Error};
use std::cell::{RefCell, Ref};
use std::clone::Clone;

pub trait RegexStruct {
    fn as_regex(&self) -> Ref<Regex>;
    fn find(&self, text: &str) -> Self;
}

pub struct Restruct;

impl Restruct {
    pub fn fill<T: RegexStruct>(regexstruct: &T, text: &str) -> T {
        regexstruct.find(text)
    }
}

#[macro_export]
macro_rules! regexify {
    ($name : ident {
        $($field : ident, $field_type : ty, $pattern : expr)*
    }) => {
        struct $name {
            $($field: $field_type,)*
            _regex : RefCell<Result<Regex, Error>>
        }

        impl Default for $name {
          fn default() -> $name {
            
            let mut regex = String::from("");
              
              $(
                   let capture_name = format!("?P<{}>", stringify!($field));
                   regex.push('('); 
                   regex.push_str(&capture_name);
                   regex.push_str($pattern);
                   regex.push(')');
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
      
              let mut i = 0;
              
              let mut filled : $name  = Default::default();
              
              $(
                  i += 1;
                  filled.$field = captures.at(i).unwrap().parse::<$field_type>().unwrap();                
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
    fn single_struct_with_same_types() {

        regexify!(HostName {
        domain, String, r"\w+"
        dot, String, r"\."
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
        ws, String, "\\s+"
        open_paren, String, r"\("
        year, i32, r"\d+"
        close_paren, String, r"\)"
      });

        let movie: Movies = Default::default();

        let filled_movie = Restruct::fill(&movie, "Not my favorite movie: 'Citizen Kane' (1941).");

        assert_eq!(r"'Citizen Kane'", filled_movie.title);
        assert_eq!(1941, filled_movie.year);
    }
}
