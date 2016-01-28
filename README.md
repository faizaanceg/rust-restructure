## Match regular expressions into struct fields

This crate is inspired from [alexflint/go-restructure](https://github.com/alexflint/go-restructure)

This crate uses a macro `regexify!` which takes the struct along with its fields and patterns for sub-expressions.

```rust
extern crate regex-struct;

use regex-struct::{Restruct, RegexStruct};

regexify!(HostName {
  domain, String, r"\w+"
  dot, String, r"\."
  tld, String, r"\w+"
});

fn main() {
  let host: HostName = Default::default();

  let filled_host = Restruct::find(&host, "example.com");

  assert_eq!("example", filled_host.domain);
  assert_eq!("com", filled_host.tld);
}

```
For the time being, you have to declare the pattern only using raw string literals.
The regular expression that was executed was the concatenation of the struct tags:

```
^(\w+)(\.)(\w+)$
```

The first submatch was inserted into the `domain` field, second into the `dot` field and the last into `tld` field.

The general format of the macro is

```rust

regexify!( <struct name> {
	<field_name>, <field_type>, <pattern>
  .
  .
});

```

`regexify!` can deal with mixed type structs, making your work easier.

```rust

extern crate regex-struct;

use regex-struct::{Restruct, RegexStruct};

regexify!(MovieDetail {
  title, String, r"'[^']+'"
  ws, String, r"\s+"
  open_paren, String, r"\("
  year, i32, r"\d+"
  close_paren, String, r"\)"
});

fn main() {

  let movie: MovieDetail = Default::default();

  let not_my_favorite_movie = Restruct::find(&movie, "Not my favorite movie: 'Citizen Kane' (1941).");

  assert_eq!(r"'Citizen Kane'", not_my_favorite_movie.title);
  assert_eq!(1941, not_my_favorite_movie.year);
  
}

```

### What `regexify!` does
Apart from declaring the struct specified, it also implements the `RegexStruct` trait on the defined struct. It also applies the trait `std::default::Default` on the struct. 

### TODO
* Nested structs
* Better error handling
* Make it available on crates.io
