# truncrate
Rust library for intelligently truncating unicode strings!

An economical way to truncate a string to a given character count or byte-offset without 
splitting graphemes. 

Examples
---------------
Depending on the encoding of your browser '🤚🏾' will produce a dark-skinned hand. In most text editors it will look like two separate characters (🤚  🏾). 

Notice how the truncation to 1 will not break the grapheme into a yellow hand:

```
use truncrate::*;
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

let s = "🤚🏾a🤚 ";
// where "🤚🏾" = 8 bytes
assert_eq!(s.truncate_to_byte_offset(0), "");
assert_eq!(s.truncate_to_byte_offset(8), "🤚🏾");
```

Aside from truncation of a single string you can also split with unicode awareness:

```
let mut s = "🤚🏾a🤚 ";
assert_eq!(s.split_all_to_boundary(1), vec!("a", "🤚"));
assert_eq!(s.split_all_to_boundary(2), vec!("🤚🏾", "a🤚",));
```

If you wish to chain splitting patterns you can do it with the 'inplace' functions:

```
let mut s = vec!("🤚🏾a🤚 ", "🤚🏾🤚🏾🤚🏾  ");
// split different byte offsets
s.split_to_offset_inplace(9)
      .split_to_offset_inplace(8)
      .split_to_offset_inplace(10);
assert_eq!(s, vec!("🤚🏾a🤚 ", "🤚🏾", "🤚🏾", "🤚🏾", " "));
```

You can also split all of your strings to boundary with the split_all_to_boundary method:
```
let s = "🤚🏾a🤚 ";
assert_eq!(s.split_all_to_boundary(3), vec!("🤚🏾a", "🤚 "));
```


For the full functionality and further examples check out the documentation. 
