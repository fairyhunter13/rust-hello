// //! # My Crate
// //!
// //! `my_crate` is a collection of utilities to make performing certain
// //! calculations more convenient.

// ///Adds one to the given number.
// /// # Examples:
// /// ```
// /// let five = 5;
// /// assert_eq!(6, cargo_and_crates::add_one(five));
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    ///The primary colors according to RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    ///The secondary colors according to RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    ///Combines two primary colors in equal amounts to create
    ///secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
