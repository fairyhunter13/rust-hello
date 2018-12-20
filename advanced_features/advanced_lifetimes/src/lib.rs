// use std::fmt;
// Rust expects lifetime parameters for the code below.
// The reason is we have lifetime related to from Parser to Context
// and Context to str literal.
// struct Context(&str);

// struct Parser {
//     context: &Context,
// }

// impl Parser {
//     fn parse(&self) -> Result<(), &str> {
//         Err(&self.context.0[1..])
//     }
// }

// Annotating same lifetime parameter for Context and Parser.
// struct Context<'a>(&'a str);

// struct Parser<'a> {
//     context: &'a Context<'a>,
// }

// impl<'a> Parser<'a> {
//     fn parse(&self) -> Result<(), &str> {
//         Err(&self.context.0[1..])
//     }
// }

// // Lifetime of return value in parse_context function is tied to Parser instance.
// fn parse_context(context: Context) -> Result<(), &str> {
//     Parser { context: &context }.parse()
// }

//Giving different lifetime parameter to Context and Parser
//But, it is still can't be compiled yet.
// The reason is that there is no relationship of lifetime 'c and 's in the code yet.
// Rust doesn't know the relationship of 'c and 's.
// We must subtype it to guarantee that 's will live long at least as long as 'c. (lifetime subtyping)
struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// Ref defined without lifetime bounds.
// This code below will produce error when compiled.
// This is happened because rust doesn't know the real lifetime of T if T contains any references.
// struct Ref<'a, T>(&'a T);

// The fix below specifies that if T contains any references,
// the references will live at least as long as Ref.
struct Ref<'a, T: 'a>(&'a T);

// Adding 'static parameter lifebound to T for specifying that if T contains any references,
// It must live long as through the entire program.
struct StaticRef<T: 'static>(&'static T);

pub trait Red {}

pub struct Ball<'a> {
    pub diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

// Using 'static lifetime parameter.
// Default lifetime parameter ellision for trait object:
// 1. 'static
// 2. &'a Trait and &'a mut Trait = 'a
// 3. Single Clause T: 'a = 'a
// 4. Multiple Clause T: 'a = undefined (specify by yourself)
pub struct BallTest {
    pub diameter: &'static i32,
}

impl Red for BallTest {}

pub struct StrWrap<'a>(&'a str);

// Too verbose and noisy.
// pub fn foo<'a>(string: &'a str) -> StrWrap<'a> {
//     StrWrap(string)
// }

// Using elided lifetimes bound.
// pub fn foo(string: &str) -> StrWrap<'_> {
//     StrWrap(string)
// }

// Automatically elided: no need to add anonymous lifetime.
pub fn foo(string: &str) -> StrWrap {
    StrWrap(string)
}

// verbose
// impl<'a> fmt::Debug for StrWrap<'a> {

// elided
// impl fmt::Debug for StrWrap<'_> {
