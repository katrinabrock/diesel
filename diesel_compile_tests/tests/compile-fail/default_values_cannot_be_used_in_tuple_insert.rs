#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::query_builder::IncompleteInsertStatement;

table! {
    users {
        id -> Integer,
        name -> Text,
        hair_color -> Text,
    }
}

fn main() {
    use users::dsl::*;

    // UFCS so rustc doesn't think we mean `Into::into`
    IncompleteInsertStatement::into(
    //~^ ERROR UndecoratedInsertRecord
    //~| ERROR E0277
        insert(&(default_values(), name.eq("Sean"))),
        users,
    );
}
