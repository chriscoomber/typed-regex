//! Class-based static implementations of each literal. Since the contents of this is always the
//! literal itself (except for case-insensitive ones) the types don't have any contents.
use syntax::LiteralChar;

#[derive(Debug)]
pub struct A;
impl LiteralChar for A {}
#[derive(Debug)]
pub struct B;
impl LiteralChar for B {}
#[derive(Debug)]
pub struct n;
impl LiteralChar for n {}
#[derive(Debug)]
pub struct u;
impl LiteralChar for u {}
#[derive(Debug)]
pub struct l;
impl LiteralChar for l {}
#[derive(Debug)]
pub struct Dash;
impl LiteralChar for Dash {}
