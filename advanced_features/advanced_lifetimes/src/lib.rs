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
