// extern crate traits;
// use traits::NewsArticle;

// #[derive(Debug)]
fn main() {
  let article = NewsArticle {
    headline: String::from("this is headline"),
    location: String::from("this is location"),
    author: String::from("this is author"),
    content: String::from("this is content"),
  };

  println!("{}", article.summarize());


  let tweet = Tweet {
    username: String::from("this is username"),
    content: String::from("this is content"),
    reply: false,
    retweet: false,
  };
println!("{}", tweet.summarize());


  
}



pub trait Summary {
  fn summarize(&self) -> String {
      String::from("this is default summarize")
  }

  fn summarize_author(&self) -> String;
  
}

pub struct NewsArticle{
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String{
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
      format!("@{}", self.username)
    }
}





// // ------------------------------------
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
