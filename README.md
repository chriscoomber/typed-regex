# Purpose

This crate is attempting to be a strongly-typed version (and otherwise a rip-off) of
<<https://github.com/rust-lang/regex/tree/master/regex-syntax>>. What do I mean by strongly-typed in this context? Well,
Eventually, I'd like to be able to do things like:

```rust
// Defines a type called `Cidr`. This implements `serde_deserialize`.
typed_regex!(Cidr, r"[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\/[0-9]{1,2}")

let test_cidr = "154.37.0.0/16";

// Parse
let cidr: Cidr = Cidr::parse(test_cidr).unwrap();

// The object this returns is an instance of the structure, with the test string encoded in it.
assert_eq!(test_cidr, cidr.to_string());

// In particular, we haven't forgotten the regex structure, and we can do further regex matches without the chance of
// failure. Here, `split2<T> is a method defined on any regex structure with at least one `T` guaranteed in it - it
// extracts the first two results of `split(T::to_char)`. This cannot fail, as this information is encoded in the types.
// One cannot do this with the current `regex_syntax` crate.
let mask_len = cidr.split2::<typed_regex::literals::slash>().second();
assert_eq!("16", mask_len.to_string());

// We can also apply further matches in a guaranteed way
typed_regex!(Ip, r"[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}")
let subnet_base = Ip.parse_guaranteed::<Cidr>(cidr);

typed_regex!(Word, r"[a-zA-Z]+")
let name = Word.parse_guaranteed::<Cidr>(cidr); // Compiler error - the regexes didn't match so `parse_guaranteed<Cidr>` is not implemented for Word
```

In some cases, such as CIDRs, it's a bit of an anti-pattern to use regex, and a custom `CIDR` structure should be used
instead. However, this crate is still useful for the general case. The ability to do further regex operations (split,
get match groups, perform further searches) using interfaces that cannot fail (i.e. they are only implemented for a
regex structure if they cannot fail for an instance of that structure) seems useful. The example here is converting a
whole CIDR to just the mask length, but another example is extracting the first name from a full name, or the year from
a date. As long as you know the regex pattern for both objects, this crate may give you a guaranteed way to move from
one to the other, without needing to define ones own structs.

# Current state

Totally not finished. Just trying to lay out how the structure would look

## Issues

- repeater is done incorrectly
- some of the code that hasn't been written yet might be impossible

## TODO

- almost all of the code
- serde integration
- json schema / swagger integration - it would be cool if the json_schema "pattern" property described Rust objects that
    this crate generated, in some way
- parser
- define all literals
- errors
- character classes/ranges (is there a way we can keep the information about the range in the type signature?, so that
    e.g. a string matching `"0-9"` is also guaranteed to match `"0-8"`.
- unicode
- tests
- proper docs
- move examples to example binary, not main
- basically everything

# Contributing

This seems like a hard problem. At the moment, only the basic idea has been set out, so there's a lot to do to make this
close to reality - including thinking more about it...

I think it shows some promise though. Feel free to submit merge requests - standard github usage.
