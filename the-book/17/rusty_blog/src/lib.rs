pub struct Post {
}
impl Post {
    pub fn new<'a> () -> &'a str{
        ""
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
