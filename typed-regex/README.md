# Features

- Patterns
  - literal
  - unions
  - concat
  - negation (hard)
  - whitespace, digit, etc.
  - repeater
- find all, find first (pattern in string), split
- match (pattern == string), get group info

# Use cases

```rust
#[macro_use]
extern crate typed_regex_derive;
extern crate typed_regex;

fn main() {
    // Derive a structure to represent the regex pattern. This structure only has 
    // associated functions, and is pointless to instantiate.
    // (Note: this is a hacky way to use procedural macros in stable rust - use a 
    // custom derive.)
    //
    // This pattern is associated with an actual pattern type (usually a `hlist::Cons`).
    #[derive(PatternBuilder)]
    #[pattern = "The sky is (gr[ea]y)"]
    struct Pattern;

    // Create an instance of the pattern type from a particular match string. If this 
    // succeeds, the resulting object encodes all the information about the matching
    // part of the string into the pattern type.
    let res = Pattern::compile_match("The sky is grey today");
    assert!(res.is_ok());
    
    // The matched part of the string can be recovered.
    let ok_res = res.unwrap();
    assert_eq!("The sky is grey", ok_res.get_matched_string());
    
    // Groups can be found. Notice how this method cannot fail to find the group, since
    // there was a group which is always captured in the pattern.
    assert_eq!("grey", ok_res.find_group_1().get_matched_string());
    
    // Other groups cannot be found. The following would not compile, as there is no
    // second group.
    // println!("{:?}", ok_res.find_group_2())

    // (Not implemented yet)
    // Can do further work, such as split on known substrings, and be sure of the type 
    // in advance, and be sure it won't fail. In this case, [&str; 4].
    assert_eq!(["The", "sky", "is", "grey"], ok_res.split::<typed_regex::Space>());
}
```