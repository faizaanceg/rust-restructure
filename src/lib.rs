extern crate regex;

use regex::Regex;

pub trait RegexStruct {
    fn to_regex(&self) -> String;
    fn fill(&self, text: &str) -> Self;
}

pub struct Restruct;

impl Restruct {
    pub fn find<T: RegexStruct>(regexstruct: &T, text: &str) -> T {
        regexstruct.fill(text)
    }
}

#[macro_export]
macro_rules! regexify {
    ($name : ident {
        $($field : ident, $field_type : ty, $pattern : expr)*
    }) => {
        struct $name {
            $($field: $field_type,)*
        }

        impl Default for $name {
          fn default() -> $name {
            $name {
              $($field : Default::default(),)*
            } 
          }
        }
                
        impl RegexStruct for $name {
            
            fn to_regex(&self) -> String {           
              
              let mut regex = String::from("");
              
              $(
                   regex.push('(');
                   regex.push_str($pattern);
                   regex.push(')');
              )*
              
              regex
            }
            
            fn fill(&self, text: &str) -> $name {
              
              let captures = Regex::new(&self.to_regex()).unwrap().captures(text).unwrap();
      
              let mut i = 0;
              
              let mut filled : $name  = Default::default();
              
              $(
                  i += 1;
                  filled.$field = captures[i].parse::<$field_type>().unwrap();                
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
    fn single_struct_with_same_types() {

        regexify!(HostName {
        domain, String, r"\w+"
        dot, String, r"\."
        tld, String, r"\w+"
      });

        let host: HostName = Default::default();

        let filled_host = Restruct::find(&host, "example.com");

        assert_eq!("example", filled_host.domain);
        assert_eq!("com", filled_host.tld);
    }

    #[test]
    fn single_struct_with_diff_types() {

        regexify!(Movies {
        title, String, r"'[^']+'"
        ws, String, r"\s+"
        open_paren, String, r"\("
        year, i32, r"\d+"
        close_paren, String, r"\)"
      });

        let movie: Movies = Default::default();

        let filled_movie = Restruct::find(&movie, "Not my favorite movie: 'Citizen Kane' (1941).");

        assert_eq!(r"'Citizen Kane'", filled_movie.title);
        assert_eq!(1941, filled_movie.year);
    }
}
