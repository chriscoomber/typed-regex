//! Features:
//!
//! - match test string against pattern
//!   - becomes an instance of the patter - useful for future failure-free computations, such as:
//!     - getting first group in an impossible to fail way
//!     - splitting on known letters to a known number of substrings (not implemented yet)
//!
//! Limitations:
//!
//! - not complete alphabet yet (extremely incomplete)
//! - only one group supported
//! - doesn't understand any shorthand specifiers (e.g. \w, \s)
//! - code is a mess
extern crate hlist;
extern crate either;
extern crate void;

use std::marker::PhantomData;

// Re-export useful things
pub use hlist::{Nil, Cons};
pub use either::Either;
pub use void::Void;

// Literals
// TODO: more!
#[derive(Debug)]
pub struct A;
impl Pattern for A {
    fn get_matched_string(&self) -> String {
        "A".to_string()
    }
}

#[derive(Debug)]
pub struct B;
impl Pattern for B {
    fn get_matched_string(&self) -> String {
        "B".to_string()
    }
}

#[derive(Debug)]
pub struct C;
impl Pattern for C {
    fn get_matched_string(&self) -> String {
        "C".to_string()
    }
}

// Shorthands
// TODO: more!
//type WordChar = Either<A, Either<B, Void>>;

// Indexes
// TODO: more!
#[derive(Debug)]
pub struct _1;
#[derive(Debug)]
pub struct _2;
#[derive(Debug)]
pub struct _3;

// TODO support empty groups?
#[derive(Debug)]
pub struct Group<GroupIndex, T> {
    pub concat: T,
    _phantoms: PhantomData<GroupIndex>
}

impl<GroupIndex, T> Group<GroupIndex, T> {
    fn new(concat: T) -> Self {
        Self { concat, _phantoms: PhantomData }
    }
}

impl<GroupIndex, T: Pattern> Pattern for Group<GroupIndex, T> {
    fn get_matched_string(&self) -> String {
        self.concat.get_matched_string()
    }
}

pub trait Pattern {
    fn get_matched_string(&self) -> String;
}

impl Pattern for Void {
    fn get_matched_string(&self) -> String {
        unreachable!()
    }
}

impl Pattern for Nil {
    fn get_matched_string(&self) -> String {
        "".to_string()
    }
}

impl<T: Pattern, S: Pattern> Pattern for Cons<T, S> {
    fn get_matched_string(&self) -> String {
        self.0.get_matched_string() + &self.1.get_matched_string()
    }
}

impl<T: Pattern, S: Pattern> Pattern for Either<T, S> {
    fn get_matched_string(&self) -> String {
        match *self {
            Either::Left(ref x) => x.get_matched_string(),
            Either::Right(ref x) => x.get_matched_string(),
        }
    }
}

pub trait Match: Sized {
    fn matches(input: &str) -> Result<(Self, u8), ()>;
}

impl Match for A {
    fn matches(input: &str) -> Result<(Self, u8), ()> {
        if input.chars().next() == Some('A') {
            return Ok((A {}, 1));
        }
        Err(())
    }
}

impl Match for B {
    fn matches(input: &str) -> Result<(Self, u8), ()> {
        if input.chars().next() == Some('B') {
            return Ok((B {}, 1));
        }
        Err(())
    }
}

impl Match for C {
    fn matches(input: &str) -> Result<(Self, u8), ()> {
        if input.chars().next() == Some('C') {
            return Ok((C {}, 1));
        }
        Err(())
    }
}

impl<GroupIndex, T: Match> Match for Group<GroupIndex, T> {
    fn matches(input: &str) -> Result<(Self, u8), ()> {
        T::matches(input).map(|(t, i)| (Group::new(t), i))
    }
}

impl Match for Nil {
    fn matches(_input: &str) -> Result<(Self, u8), ()> {
        Ok((Nil {}, 0))
    }
}

impl<T: Match, S: Match> Match for Cons<T, S> {
    fn matches(input: &str) -> Result<(Self, u8), ()> {
        // Match against the first, and whatever's left give to second
        if let Ok((t, i)) = T::matches(input) {
            if let Ok((s, j)) = S::matches(&input[i as usize..]) {
                return Ok((Cons(t, s), i+j));
            }
        }
        Err(())
    }
}

impl Match for Void {
    fn matches(_input: &str) -> Result<(Self, u8), ()> {
        Err(())
    }
}

impl<T: Match, S: Match> Match for Either<T, S> {
    fn matches(input: &str) -> Result<(Self, u8), ()> {
        if let Ok((t, i)) = T::matches(input) {
            return Ok((Either::Left(t), i))
        }
        if let Ok((s, j)) = S::matches(input) {
            return Ok((Either::Right(s), j))
        }
        Err(())
    }
}

#[allow(dead_code)]
pub enum Here {}
#[allow(dead_code)]
pub struct InFirstBranch<T>(PhantomData<T>);
#[allow(dead_code)]
pub struct InSecondBranch<T>(PhantomData<T>);

// TODO: more groups?
/// This is a bit of a trick. I'd like to implement `FindGroup1<U>` for
/// `Concat<T, S>; T: FindGroup1<U>` one way (look in first element) and for
/// `Concat<T, S>; S: FindGroup1<U>` another way (look in second element). However, this isn't
/// possible due to specialization (we can't convince the compiler that these won't both be
/// implemented). So, instead we implement two different traits that happen to have the same method
/// name. There's only one Group 1, so it's not possible for these to both be implemented for the
/// same structure, so `x.find_group_1()` will just work.
///
/// (This trick was borrowed from the hlist crate.)
pub trait FindGroup1<U, Where> {
    fn find_group_1(&self) -> &U;
}

impl<U> FindGroup1<U, Here> for Group<_1, U> {
    fn find_group_1(&self) -> &U {
        &self.concat
    }
}

impl<GroupIndex, T, U, Where> FindGroup1<U, InFirstBranch<Where>> for Group<GroupIndex, T>
    where T: FindGroup1<U, Where>
{
    fn find_group_1(&self) -> &U {
        self.concat.find_group_1()
    }
}

impl<T, S, U, Where> FindGroup1<U, InFirstBranch<Where>> for Cons<T, S>
    where T: FindGroup1<U, Where>
{
    fn find_group_1(&self) -> &U {
        &self.0.find_group_1()
    }
}

impl<T, S, U, Where> FindGroup1<U, InSecondBranch<Where>> for Cons<T, S>
    where S: FindGroup1<U, Where>
{
    fn find_group_1(&self) -> &U {
        &self.1.find_group_1()
    }
}

/// Either can actually fail to find the group
pub trait MaybeFindGroup1<U, Where> {
    fn maybe_find_group_1(&self) -> Option<&U>;
}

// tediously re-implement the weaker search in all the same places
impl<U> MaybeFindGroup1<U, Here> for Group<_1, U> {
    fn maybe_find_group_1(&self) -> Option<&U> {
        Some(&self.concat)
    }
}

impl<GroupIndex, T, U, Where> MaybeFindGroup1<U, InFirstBranch<Where>> for Group<GroupIndex, T>
    where T: MaybeFindGroup1<U, Where>
{
    fn maybe_find_group_1(&self) -> Option<&U> {
        self.concat.maybe_find_group_1()
    }
}

impl<T, S, U, Where> MaybeFindGroup1<U, InFirstBranch<Where>> for Cons<T, S>
    where T: MaybeFindGroup1<U, Where>
{
    fn maybe_find_group_1(&self) -> Option<&U> {
        self.0.maybe_find_group_1()
    }
}

impl<T, S, U, Where> MaybeFindGroup1<U, InSecondBranch<Where>> for Cons<T, S>
    where S: MaybeFindGroup1<U, Where>
{
    fn maybe_find_group_1(&self) -> Option<&U> {
        self.1.maybe_find_group_1()
    }
}

impl<T, S, U, Where> MaybeFindGroup1<U, InFirstBranch<Where>> for Either<T, S>
    where T: MaybeFindGroup1<U, Where>
{
    fn maybe_find_group_1(&self) -> Option<&U> {
        match *self {
            Either::Left(ref x) => x.maybe_find_group_1(),
            Either::Right(_) => None,
        }
    }
}

impl<T, S, U, Where> MaybeFindGroup1<U, InSecondBranch<Where>> for Either<T, S>
    where S: MaybeFindGroup1<U, Where>
{
    fn maybe_find_group_1(&self) -> Option<&U> {
        match *self {
            Either::Left(_) => None,
            Either::Right(ref x) => x.maybe_find_group_1(),
        }
    }
}

#[cfg(test)]
#[macro_use]
extern crate typed_regex_derive;

#[cfg(test)]
mod tests {
    extern crate typed_regex;

    use self::typed_regex::{Pattern, FindGroup1, MaybeFindGroup1};

    #[test]
    fn concat() {
        #[derive(PatternBuilder)]
        #[pattern = "AB"]
        struct Pattern;

        let res = Pattern::compile_match("AB");
        assert!(res.is_ok());
        assert_eq!("AB", res.as_ref().unwrap().get_matched_string());

        let res = Pattern::compile_match("AA");
        assert!(res.is_err());
    }

    #[test]
    fn match_just_start() {
        #[derive(PatternBuilder)]
        #[pattern = "AB"]
        struct Pattern;

        let res = Pattern::compile_match("ABCCCCCCCCCCCCCCCCCCC");
        assert!(res.is_ok());
        assert_eq!("AB", res.as_ref().unwrap().get_matched_string());
    }

    #[test]
    fn match_too_long() {
        #[derive(PatternBuilder)]
        #[pattern = "ABCCCCCCCCCCCCCCCCCCC"]
        struct Pattern;

        let res = Pattern::compile_match("AB");
        assert!(res.is_err());
    }

    #[test]
    fn altern() {
        #[derive(PatternBuilder)]
        #[pattern = "[AB]"]
        struct Pattern;

        let res = Pattern::compile_match("A");
        assert!(res.is_ok());
        assert_eq!("A", res.as_ref().unwrap().get_matched_string());

        let res = Pattern::compile_match("B");
        assert!(res.is_ok());
        assert_eq!("B", res.as_ref().unwrap().get_matched_string());

        let res = Pattern::compile_match("C");
        assert!(res.is_err());
    }

    #[test]
    fn altern_and_concat() {
        #[derive(PatternBuilder)]
        #[pattern = "C[AB]C"]
        struct Pattern;

        let res = Pattern::compile_match("CAC");
        assert!(res.is_ok());
        assert_eq!("CAC", res.as_ref().unwrap().get_matched_string());

        let res = Pattern::compile_match("CBC");
        assert!(res.is_ok());
        assert_eq!("CBC", res.as_ref().unwrap().get_matched_string());

        let res = Pattern::compile_match("CCC");
        assert!(res.is_err());
    }

    #[test]
    fn first_group() {
        #[derive(PatternBuilder)]
        #[pattern = "A(BC)(A)"]
        struct Pattern;

        let res = Pattern::compile_match("ABCA");
        assert!(res.is_ok());
        assert_eq!("ABCA", res.as_ref().unwrap().get_matched_string());
        assert_eq!("BC", res.as_ref().unwrap().find_group_1().get_matched_string());
        assert_eq!("BC", res.as_ref().unwrap().maybe_find_group_1().unwrap().get_matched_string());
    }

    #[test]
    fn alterns_of_groups() {
        #[derive(PatternBuilder)]
        #[pattern = "A[(BBB)(CCC)]A"]
        struct Pattern;

        let res = Pattern::compile_match("ABBBA");
        assert!(res.is_ok());
        assert_eq!("ABBBA", res.as_ref().unwrap().get_matched_string());
        assert_eq!("BBB", res.as_ref().unwrap().maybe_find_group_1().unwrap().get_matched_string());

        let res = Pattern::compile_match("ACCCA");
        assert!(res.is_ok());
        assert_eq!("ACCCA", res.as_ref().unwrap().get_matched_string());
        assert!(res.as_ref().unwrap().maybe_find_group_1().is_none());
    }

    #[test]
    fn groups_of_alterns() {
        #[derive(PatternBuilder)]
        #[pattern = "A([BC][BC])A"]
        struct Pattern;

        let res = Pattern::compile_match("ABCA");
        assert!(res.is_ok());
        assert_eq!("ABCA", res.as_ref().unwrap().get_matched_string());
        assert_eq!("BC", res.as_ref().unwrap().maybe_find_group_1().unwrap().get_matched_string());
    }
}