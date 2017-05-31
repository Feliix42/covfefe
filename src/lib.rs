use std::string::String;

pub trait Covfefe {
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
