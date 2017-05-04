//! The `restructure` crate provides functionalities to match regexp patterns
//! into struct fields. This crate now builds on **rust-stable**.
//!
//! To use this, add the following to your `Cargo.toml`.
//!
//!```text
//! [dependencies]
//! restructure = "0.1.1"
//!```
//!
//! and add this to your crate root:
//!
//!```
//! extern crate regex;
//!```
extern crate regex;

/// This trait allows you to match struct fields with regexp
pub trait RegexStruct {
    /// This function takes a Slice, find Captures in it and assigns
    /// it to the appropriate struct fields
    fn find(&self, text: &str) -> Self;
}

/// Contains the `fill` method used to fill the struct with fields
pub struct Restruct;

impl Restruct {
    /// This function takes a `RegexStruct` and a `Slice` and returns a `RegexStruct`
    /// with its fields filled with
    /// the patterns from the text
    ///
    ///```
    /// # #[macro_use] extern crate restructure;
    /// # extern crate regex;
    /// # use regex::Regex;
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
/// # #[macro_use] extern crate restructure;
/// # extern crate regex;
/// # use regex::Regex;
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
            _regex : Regex
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
                       let named_capture = format!("(?P<{}>{})", y, $pattern);
                       regex.push_str(&named_capture);
                    }
                   }
              )*

            $name {
              $($field : Default::default(),)*
              _regex : Regex::new(regex.as_str()).unwrap()
            }
          }
        }

        impl RegexStruct for $name {

            // f`

            fn find(&self, text: &str) -> $name {

              let captures = self._regex.captures(text).unwrap();

              let mut filled : $name  = Default::default();

              $(
                  if let Some(value) = captures.name(stringify!($field)) {
                    filled.$field = value.as_str().parse::<$field_type>().unwrap();
                  }
              )*

              filled
            }
        }
    }
}

#[cfg(test)]
mod test {

    use regex::Regex;
    use super::{Restruct, RegexStruct};

    #[test]
    fn single_struct_regex() {

        regexify!(SemVer {
        major, i32, r"\d+"
        __1, String, r"\."
        minor, i32, r"\d+"
        __2, String, r"\."
        patch, i32, r"\d+"
      });

        let version: SemVer = Default::default();

        assert_eq!(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)",
                   version._regex.as_str());
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
