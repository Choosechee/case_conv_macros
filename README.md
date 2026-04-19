Convert identifiers and string literals to different cases
## Motivation
I wanted a macro to convert identifiers to camel case string literals for use in a project, but I only found one that converted to pascal case. So, I forked it and added support for all the case styles you could ever want, so no one else has the same problem I had. I also added support for converting string literals, because why not?
## Installation
Add it to your project with cargo (COMING SOON):
```
cargo add case_conv_macros
```
## Example
```rust
use case_conv_macros::{identifier_to_camel, literal_to_sentence};

let my_rusty_identifier = identifier_to_camel!(my_rusty_identifier);
assert_eq!(my_rusty_identifier, "myRustyIdentifier");

let my_rusty_literal = literal_to_sentence!("my_rusty_literal");
assert_eq!(my_rusty_literal, "My rusty literal");
```
