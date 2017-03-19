extern crate typed_regex;

use std::marker::PhantomData;

use typed_regex::*;

fn example_1() {
    let regex_1 = r".";

    // Macro generates this. This should be Deserialize.
    #[derive(Debug)]
    struct Regex1(Concat<AnyChar, End>);

    let test_string = "a";

    // Regex-deserializing test_string should give (if it passes):
    let x = Regex1(Concat {
        this: AnyChar('a'),
        next: End,
    });

    println!("{:?}", x);
}

fn example_2() {
    let regex_2 = r"^.{4}-(.|null)$";

    // Macro generates this. The result should impl Deserialize.
    #[derive(Debug)]
    struct Repeater4;
    impl Static<Repeater> for Repeater4 {
        fn get() -> Repeater {
            Repeater::Range {
                min: 4,
                max: None,
            }
        }
    }
    #[derive(Debug)]
    struct NotGreedy;
    impl Static<RepeatContext> for NotGreedy {
        fn get() -> RepeatContext {
            RepeatContext {
                greedy: true
            }
        }
    }
    #[derive(Debug)]
    struct Group1;
    impl Static<GroupContext> for Group1 {
        fn get() -> GroupContext {
            GroupContext {
                name: None,
                index: Some(1),
            }
        }
    }
    type NullType = CaseSensitiveLiteral<literal::n, CaseSensitiveLiteral<literal::u, CaseSensitiveLiteral<literal::l, CaseSensitiveLiteral<literal::l, End>>>>;
    #[derive(Debug)]
    struct Regex2(
        Concat<
            StartText,
            Concat<
                Repeat<AnyChar, Repeater4, NotGreedy>,
                Concat<
                    CaseSensitiveLiteral<literal::Dash, End>,
                    Concat<Group<Alternate<AnyChar, Alternate<NullType, End>>, Group1>, End>
                >
            >
        >
    );

    let test_string = "aaaa-null";

    // Regex-deserializing test_string should give (if it passes):
    // FIXME: ".{4}" can match "abcd", but my syntax can't. Oops.
    let x = Regex2(
        Concat {
            this: StartText,
            next: Concat {
                this: Repeat {
                    e: AnyChar('a'),
                    _phantoms: PhantomData,
                },
                next: Concat {
                    this: CaseSensitiveLiteral {
                        this: literal::Dash,
                        next: End,
                    },
                    next: Concat {
                        this: Group {
                            e: Alternate::Other(Alternate::This(CaseSensitiveLiteral {this: literal::n, next: CaseSensitiveLiteral {this: literal::u, next: CaseSensitiveLiteral {this: literal::l, next: CaseSensitiveLiteral {this: literal::l, next: End}}}})),
                            _phantoms: PhantomData,
                        },
                        next: End,
                    }
                }
            }
        }
    );

    println!("{:?}", x);
}

fn main() {
    example_1();
    example_2();
}