#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::pg::PgConnection;
use diesel::query_builder::IncompleteInsertStatement;

table! {
    users {
        id -> Integer,
    }
}

table! {
    posts {
        id -> Integer,
    }
}

fn main() {
    let conn = PgConnection::establish("").unwrap();

    insert(&posts::id.eq(1))
        .into(users::table)
        //~^ ERROR mismatched types
        .execute(&conn)
        .unwrap();

    // UFCS to enusre it doesn't think we meant `Into::into`
    IncompleteInsertStatement::into(
        &(posts::id.eq(1), users::id.eq(2)),
        //~^ ERROR mismatched types
        users::table,
    )
}
