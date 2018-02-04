//! This crate adds the `PatternBuilder` macro.
//!
//! ```rust
//! #[macro_use]
//! extern crate typed_regex_derive;
//!
//! #[derive(PatternBuilder)]
//! #[pattern = "A[BA][BA]A"]
//! struct AbbaPattern;
//!
//! fn main() {
//!     let res = FirstPattern::compile_match("ABBAAAAAA");
//!     assert_eq!("ABBA", res.unwrap().get_matched_string());
//! }
//! ```

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(PatternBuilder, attributes(pattern))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Extract the pattern literal from the annotations
    let pattern_literal = ast.attrs.iter().filter_map(|attr| match attr.value {
        syn::MetaItem::NameValue(ref ident, syn::Lit::Str(ref s, _)) if ident == "pattern" => Some(s.clone()),
        _ => None,
    }).last().expect("Invalid or absent pattern literal");

    // Build the impl
    let gen = impl_pattern_builder(&ast, pattern_literal);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_pattern_builder(ast: &syn::DeriveInput, pattern_literal: String) -> quote::Tokens {
    let name = &ast.ident;

    // TODO be less stupid about utf8
    let mut chars: Vec<char> = pattern_literal.chars().map(|x| x as char).collect();

    // Every regex is a concat at the outer level
    let type_tokens = pop_concat_type_tokens(&mut chars);

    let dummy_const = syn::Ident::new(format!("_IMPL_PATTERN_BUILDER_FOR_{}", name));

    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate typed_regex as _typed_regex;

            impl #name {
                fn compile_match(input: &str) -> Result<#type_tokens, ()> {
                    <#type_tokens as _typed_regex::Match>::matches(input).map(|(t, _)| t)
                }
            }
        };
    }
}

fn pop_concat_type_tokens(chars: &mut Vec<char>) -> quote::Tokens {
    let mut tokens = quote!(_typed_regex::Nil);
    while !chars.is_empty() {
        let new_token = match chars.pop() {
            Some('A') => quote!(_typed_regex::A),
            Some('B') => quote!(_typed_regex::B),
            Some('C') => quote!(_typed_regex::C),
            Some(']') => {
                // new altern
                pop_altern_type_tokens(chars)
            },
            Some(')') => {
                // new group
                // TODO: This is a bit shit - counts down instead of up
                // find out the index of the group - should be the number of ')' left + 1
                let index = chars.iter().fold(0, |acc, x| {
                    if *x == ')' {
                        acc + 1
                    } else {
                        acc
                    }
                }) + 1;

                let group = pop_concat_type_tokens(chars);

                match index {
                    1 => quote!(_typed_regex::Group<_typed_regex::_1, #group>),
                    2 => quote!(_typed_regex::Group<_typed_regex::_2, #group>),
                    3 => quote!(_typed_regex::Group<_typed_regex::_3, #group>),
                    _ => panic!("I can't count that high"),
                }
            },
            Some('(') => {
                // end of group
                return tokens;
            }
            Some('[') => {
                // end of altern - something has gone wrong
                panic!("altern/group interleaved")
            }
            Some(_) => panic!("Unrecognized character"),
            None => unreachable!(),
        };

        tokens = quote!(_typed_regex::Cons<#new_token, #tokens>)
    }

    return tokens;
}

/// Pop chars off of the pattern until we have a new set of altern type tokens
fn pop_altern_type_tokens(chars: &mut Vec<char>) -> quote::Tokens {
    let mut tokens = quote!(_typed_regex::Void);
    while !chars.is_empty() {
        let new_token = match chars.pop() {
            Some('A') => quote!(_typed_regex::A),
            Some('B') => quote!(_typed_regex::B),
            Some('C') => quote!(_typed_regex::C),
            Some(']') => {
                // new altern
                pop_altern_type_tokens(chars)
            },
            Some(')') => {
                // new group
                // TODO: This is a bit shit - counts down instead of up
                // find out the index of the group - should be the number of ')' left + 1
                let index = chars.iter().fold(0, |acc, x| {
                    if *x == ')' {
                        acc + 1
                    } else {
                        acc
                    }
                }) + 1;

                let group = pop_concat_type_tokens(chars);

                match index {
                    1 => quote!(_typed_regex::Group<_typed_regex::_1, #group>),
                    2 => quote!(_typed_regex::Group<_typed_regex::_2, #group>),
                    3 => quote!(_typed_regex::Group<_typed_regex::_3, #group>),
                    _ => panic!("I can't count that high"),
                }
            },
            Some('(') => {
                // end of group - something has gone wrong
                panic!("altern/group interleaved")
            }
            Some('[') => {
                // end of altern
                return tokens;
            }
            Some(_) => panic!("Unrecognized character"),
            None => unreachable!(),
        };

        tokens = quote!(_typed_regex::Either<#new_token, #tokens>)
    }

    return tokens;
}