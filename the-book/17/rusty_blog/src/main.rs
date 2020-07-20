use rusty_blog::Post;

fn main(){
  let post = Post::new();
  post.add_text("hey");
  assert_eq!("", post.content()); // << compile errorで保証
}