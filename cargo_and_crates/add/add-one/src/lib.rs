extern crate rand;

pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
