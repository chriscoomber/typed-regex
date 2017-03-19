//! Definitions for each of the expression objects.
//! The idea here is that each `Expr` struct defines the matching expression, and an instance of
//! that class describes the string that matched.
use std::marker::PhantomData;
use std::fmt::Debug;

/// Many of the `Expr` implementations below can not reasonably be expressed by structs - many of
/// them have contexts such as ranges or flags which would be tedious to capture in the type system.
/// So, instead, we generate structs just-in-time to represent these contexts in a static way.
/// Those structs implement this trait, to provide a way to extract the context information.
pub trait Static<T>: Debug {
    fn get() -> T;
}

/// Common end for array-like things
#[derive(Debug)]
pub struct End;

pub trait Expr: Debug {}

/// An empty regex (which never matches any text).
#[derive(Debug)]
pub struct Empty;
impl Expr for Empty {}

/// A sequence of one or more literal characters to be matched.
#[derive(Debug)]
pub struct CaseSensitiveLiteral<T: LiteralChar, U: LiteralCharArray> {
    pub this: T,
    pub next: U,
}
impl<T: LiteralChar, U: LiteralCharArray> Expr for CaseSensitiveLiteral<T, U> {}
pub trait LiteralChar: Debug {}
pub trait LiteralCharArray: Debug {}
impl<T: LiteralChar, U: LiteralCharArray> LiteralCharArray for CaseSensitiveLiteral<T, U> {}
impl LiteralCharArray for End {}

/// Case insensitive literal
//pub struct CaseInsensitiveLiteral<T: CaseInsensitiveLiteralChar, U: CaseInsensitiveLiteralCharArray>;

/// Match any character.
#[derive(Debug)]
pub struct AnyChar(pub char);
impl Expr for AnyChar {}

/// Match any character, excluding new line (`0xA`).
#[derive(Debug)]
pub struct AnyCharNoNL(pub char);
impl Expr for AnyCharNoNL {}

/// A character class.
// #[derive(Debug)]
//pub struct Class(CharClass);
//impl Expr for Class {}

/// Match the start of a line or beginning of input.
#[derive(Debug)]
pub struct StartLine;
impl Expr for StartLine {}

/// Match the end of a line or end of input.
#[derive(Debug)]
pub struct EndLine;
impl Expr for EndLine {}

/// Match the beginning of input.
#[derive(Debug)]
pub struct StartText;
impl Expr for StartText {}

/// Match the end of input.
#[derive(Debug)]
pub struct EndText;
impl Expr for EndText {}

/// Match a word boundary (word character on one side and a non-word
/// character on the other).
#[derive(Debug)]
pub struct WordBoundary;
impl Expr for WordBoundary {}

/// Match a position that is not a word boundary (word or non-word
/// characters on both sides).
#[derive(Debug)]
pub struct NotWordBoundary;
impl Expr for NotWordBoundary {}

/// A group, possibly non-capturing.
#[derive(Debug)]
pub struct Group<T: Expr, C: Static<GroupContext>> {
    /// The expression inside the group.
    pub e: T,
    pub _phantoms: PhantomData<C>,
}
impl<T: Expr, C: Static<GroupContext>> Expr for Group<T, C> {}
/// Context (name, capture index) for a group.
#[derive(Debug)]
pub struct GroupContext {
    pub name: Option<String>,
    pub index: Option<u16>,
}

/// A repeat operator (`?`, `*`, `+` or `{m,n}`).
#[derive(Debug)]
pub struct Repeat<T: Expr, R: Static<Repeater>, C: Static<RepeatContext>> {
    /// The expression to be repeated. Limited to literals, `.`, classes
    /// or grouped expressions.
    pub e: T,
    pub _phantoms: PhantomData<(R, C)>,
}
impl<T: Expr, R: Static<Repeater>, C: Static<RepeatContext>> Expr for Repeat<T, R, C> {}
/// Context (name, capture index) for a repeat match.
#[derive(Debug)]
pub struct RepeatContext {
    pub greedy: bool,
}
/// The type of a repeat operator expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repeater {
    /// Match zero or one (`?`).
    ZeroOrOne,
    /// Match zero or more (`*`).
    ZeroOrMore,
    /// Match one or more (`+`).
    OneOrMore,
    /// Match for at least `min` and at most `max` (`{m,n}`).
    ///
    /// When `max` is `None`, there is no upper bound on the number of matches.
    Range {
        /// Lower bound on the number of matches.
        min: u32,
        /// Optional upper bound on the number of matches.
        max: Option<u32>,
    },
}
impl Repeater {
    /// Returns true if and only if this repetition can match the empty string.
    pub fn matches_empty(&self) -> bool {
        use self::Repeater::*;
        match *self {
            ZeroOrOne => true,
            ZeroOrMore => true,
            OneOrMore => false,
            Range { min, .. } => min == 0,
        }
    }
}

/// A concatenation of expressions. Must be matched one after the other.
///
/// N.B. A concat expression can only appear at the top-level or
/// immediately inside a group expression.
#[derive(Debug)]
pub struct Concat<T: Expr, U: ExprArray> {
    pub this: T,
    pub next: U,
}
pub trait ExprArray: Debug {}
impl<T: Expr, U: ExprArray> ExprArray for Concat<T, U> {}
impl<T: Expr, U: ExprArray> Expr for Concat<T, U> {}
impl ExprArray for End {}

/// An alternation of expressions. Only one must match.
///
/// N.B. An alternate expression can only appear at the top-level or
/// immediately inside a group expression.
#[derive(Debug)]
pub enum Alternate<T: Expr, U: ExprAlternate> {
    This(T),
    Other(U),
}
pub trait ExprAlternate: Debug {}
impl<T: Expr, U: ExprAlternate> ExprAlternate for Alternate<T, U> {}
impl<T: Expr, U: ExprAlternate> Expr for Alternate<T, U> {}
impl ExprAlternate for End {}
