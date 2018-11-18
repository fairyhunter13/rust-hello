fn main() {
    let n_main: String = String::from("test");
    println!("{}", inc_ref(&n_main));
}

fn inc_ref(n_inc_ref: &str) -> usize {
    n_inc_ref.len()
}
