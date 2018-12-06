extern crate cargo_and_crates_fairyhunter13 as art;

// use art::kinds::PrimaryColor;
// use art::utils::mix;
use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
