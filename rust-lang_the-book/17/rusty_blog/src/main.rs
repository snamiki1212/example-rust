use rusty_blog::Post;

fn main(){
  let mut post = Post::new();

  post.add_text("hey");

  let post = post.request_review();

  let post = post.approve();

  assert_eq!("hey", post.content());
}