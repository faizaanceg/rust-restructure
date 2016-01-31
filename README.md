## Match regular expressions into struct fields

This crate is inspired from [alexflint/go-restructure](https://github.com/alexflint/go-restructure)

This crate uses a macro `regexify!` which takes the struct along with its fields and patterns for sub-expressions.

```rust
#![feature(cell_extras)]

#[macro_use(regexify)]
extern crate restructure;
extern crate regex;

use std::cell::{RefCell, Ref};
use regex::{Regex, Error};
use restructure::{Restruct, RegexStruct};

regexify!(HostName {
  domain, String, r"\w+"
  _dot, String, r"\."
  tld, String, r"\w+"
});

fn main() {
  
  let host: HostName = Default::default();

  let filled_host = Restruct::fill(&host, "example.com");

  assert_eq!("example", filled_host.domain);
  assert_eq!("com", filled_host.tld);
}

```
The regular expression that was executed was the concatenation of the struct tags:

```
(?P<domain>\w+)\.(?P<tld>\w+)
```
You can see that the fields which start with a `_` are not added as captures into the regex. You can use `_` with fields which act as seperators or whitespace. 

The first submatch was inserted into the `domain` field and the next into `tld` field.

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
#![feature(cell_extras)]

#[macro_use(regexify)]
extern crate restructure;
extern crate regex;

use std::cell::{RefCell, Ref};
use regex::{Regex, Error};
use restructure::{Restruct, RegexStruct};

regexify!(MovieDetail {
  title, String, r"'[^']+'"
  _1, String, r"\s+\("
  year, i32, r"\d+"
  _2, String, r"\)"
});

fn main() {

  let movie: MovieDetail = Default::default();

  let not_my_favorite_movie = Restruct::fill(&movie, "Not my favorite movie: 'Citizen Kane' (1941).");

  assert_eq!(r"'Citizen Kane'", not_my_favorite_movie.title);
  assert_eq!(1941, not_my_favorite_movie.year);
  
}

```

### What `regexify!` does
Apart from declaring the struct specified, it also implements the `RegexStruct` trait on the defined struct. It also applies the trait `std::default::Default` on the struct. 

### TODO
* Nested structs
* Better error handling
* Iterable structs
* JSON conversion (optional)
