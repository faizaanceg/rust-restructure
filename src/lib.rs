extern crate regex;

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
                
        impl $name {
            
            fn to_regex(&self) -> String {           
              
              let mut regex = String::from("");
              
              $(
                   regex.push('(');
                   regex.push_str($pattern);
                   regex.push(')');
              )*
              
              regex
            }
            
            fn find(&self, text: &str) -> $name {
              
              let captures = Regex::new(&self.to_regex()).unwrap().captures(text).unwrap();
      
              let mut i = 0;
              
              let mut filled_struct : $name  = Default::default();
              
              $(
                  i += 1;
                  filled_struct.$field = captures.at(i).unwrap().to_owned().parse::<$field_type>().unwrap();                
              )*
              
              filled_struct
            } 
        }
    }
}

#[cfg(test)]
mod test {

    use regex::Regex;

    #[test]
    fn single_struct_with_same_types() {

        regexify!(HostName {
        domain, String, r"\w+"
        dot, String, r"\."
        tld, String, r"\w+"
      });

        let host: HostName = Default::default();

        let filled_host = host.find("example.com");

        assert_eq!("example", filled_host.domain);
        assert_eq!("com", filled_host.tld);
    }

    #[test]
    fn single_struct_with_diff_types() {

        regexify!(Movies {
        title, String, r"\w+"
        ws, String, r"\s+"
        open_paren, String, r"\("
        year, i32, r"\d+"
        close_paren, String, r"\)"
      });

        let movie: Movies = Default::default();

        let filled_movie = movie.find("Red (2010)");

        assert_eq!("Red", filled_movie.title);
        assert_eq!(2010, filled_movie.year);
    }
}
