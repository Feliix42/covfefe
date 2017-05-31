//! This is a small crate to end tweets or sentences with Style.
//!
//! This crate is inspired by [rgbkrk](https://github.com/rgbkrk/covfefe) and
//! the [45th president](https://archive.is/f7UL3) of the United States of America.
//! It basically consists of a _Trait_ - `Cofveve`, which is implemented for all
//! types that implement [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html).
//!
//! ## Example
//! ```
//! # use Covfefe;
//! println!("{}", "Despite the constant negative press".covfefe());
//! ```

#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]
use std::string::String;

/// The Covfefe Trait. Not everyone has it.
pub trait Covfefe {
    /// Covfefe.
    ///
    /// Terminate the input using pure knowledge and skill.
    fn covfefe(self) -> String;
}


impl<T> Covfefe for T
    where T: ToString
{
    #[inline(always)]
    fn covfefe(self) -> String {
        let mut modified_string = self.to_string();
        if modified_string.ends_with(" ") {
            modified_string.push_str("covfefe");
        } else {
            modified_string.push_str(" covfefe");
        }
        modified_string
    }
}


#[cfg(test)]
mod tests {
    use Covfefe;

    #[test]
    fn make_covfefe() {
        let test1 = String::from("Despite the constant negative press");
        assert_eq!(test1.covfefe(),
                   "Despite the constant negative press covfefe");

        let test2 = String::from("Despite the constant negative press ");
        assert_eq!(test2.covfefe(),
                   "Despite the constant negative press covfefe");
    }

    #[test]
    fn make_covfefe_str() {
        assert_eq!("Despite the constant negative press".covfefe(),
                   "Despite the constant negative press covfefe");

        assert_eq!("Despite the constant negative press ".covfefe(),
                   "Despite the constant negative press covfefe");
    }
}
