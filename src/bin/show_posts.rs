extern crate diesel;
extern crate rust_diesel;

use self::diesel::prelude::*;
use self::models::*;
use self::rust_diesel::*;

fn main() {
    use rust_diesel::schema::posts::dsl::*;

    let connection = estabilish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("---------------\n");
        println!("{}", post.body);
    }
}
