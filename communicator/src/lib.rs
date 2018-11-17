pub mod client;
// mod client {
//     fn connect() {}
// }
pub mod network;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();
    // outermost::inside::inner_function();
    // outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        client::connect();
    }
}
