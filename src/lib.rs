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
    #[test]
    fn it_works() {
    }
}
