extern crate blog;
use blog::Post;

fn main (){
    let mut post = Post::new();

    // add
    post.add_text("hey");
    assert_eq!("", post.content());

    // review
    post.request_review();
    assert_eq!("", post.content());

    // approve
    post.approve();
    assert_eq!("hey", post.content());

}