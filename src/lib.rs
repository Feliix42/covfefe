use std::string::String;

pub trait Covfefe {
    fn covfefe(self) -> String;
}


impl Covfefe for String {
    fn covfefe(self) -> String {
        let mut modified_string = String::from(self);
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
}
