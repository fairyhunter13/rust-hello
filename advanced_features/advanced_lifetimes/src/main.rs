fn main() {
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
}
