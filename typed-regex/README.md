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
extern crate typed_regex;

fn main() {
    let res: Pattern!("[AB]{2}C") = Pattern::match ;

    // Strings are encoded in the type system...
    // Produces a `Cons<A, Cons<B, Cons<C, Nil>>>` 
    let exact_test_string = compile_string!("ABC");
    let test_string = "ABC";
    
    // Produces a `Cons<Repeater2<Either<A, B>>, Cons<C, Nil>>`
    let exact_pattern = compile_pattern!("[AB]{2}C");
    // Produces a `Pattern` - note not a macro, can be used on any string.
    let inexact_pattern = compile_runtime_pattern("B");
    
    // With both exact pattern and string, can do following.
    let 
    

    // Returns a `Concat<A, Concat<Comma, Concat<B, Concat<Comma, Concat<C>>>>>`.
    let exact_pattern = compile!("a,b,c");
    
    let words: (ALower, BLower, CLower) = pattern.split(',');
    
    
    // 
}
```