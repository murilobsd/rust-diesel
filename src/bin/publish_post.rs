extern crate diesel;
extern crate rust_diesel;

use self::diesel::prelude::*;
use self::models::Post;
use self::rust_diesel::*;
use std::env::args;

fn main() {
    use rust_diesel::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("invalid id");
    let connection = estabilish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .unwrap_or_else(|_| panic!("unable to find post {}", id));

    println!("Published post {}", post.title);
}
