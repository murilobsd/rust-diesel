#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn estabilish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
