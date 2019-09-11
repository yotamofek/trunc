# trunc
Rust library for intelligently truncating unicode strings!

An economical way to truncate a string to a given character count or byte-offset without 
splitting graphemes. 

Examples
---------------
Depending on the encoding of your browser '🤚🏾' will produce a dark-skinned hand. In most text editors it will look like two separate characters. 

Notice how the truncation to 1 will not break the grapheme into a yellow hand:

```
use trunc::*;
let s = "🤚🏾a🤚 🤚🏾\t 🤚    ";

assert_eq!(s.truncate_to_boundary(1), "");
assert_eq!(s.truncate_to_boundary(2), "🤚🏾");

```


Should you set a numeric boundary which ends with a whitespace - truncation will trim the whitespace for you:

```
assert_eq!(s.truncate_to_boundary(4), "🤚🏾a🤚");
assert_eq!(s.truncate_to_boundary(5), "🤚🏾a🤚");
```

But if the truncation exceeds the strings size it will return the entire string:

```
assert_eq!(s.truncate_to_boundary(10), s);
```

You can also choose to truncate by byte-offset (i.e., byte-size boundary):

```
use trunc::*;

let s = "🤚🏾a🤚 ";
// where "🤚🏾" = 8 bytes
assert_eq!(s.truncate_to_byte_offset(0), "");
assert_eq!(s.truncate_to_byte_offset(8), "🤚🏾");
```

For further explanations and examples check out the auto generated documentation with:
```
cargo doc --open
```