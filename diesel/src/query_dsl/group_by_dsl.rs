use expression::Expression;
use query_builder::{AsQuery, Query};
use query_source::Table;

/// Adds `GROUP BY` to the query
/// # Example
///
/// ```rust
///
/// # #[macro_use] extern crate diesel;
/// # #[macro_use] extern crate diesel_codegen;
/// # include!("../doctest_setup.rs");
/// # use schema::users;
/// # use schema::posts;
/// #
/// # #[derive(Debug, PartialEq, Identifiable, Queryable)]
/// # pub struct User {
/// #     id: i32,
/// #     name: String,
/// # }
/// #
/// # #[derive(Debug, PartialEq, Identifiable, Queryable, Associations)]
/// # #[belongs_to(User)]
/// # pub struct Post {
/// #     id: i32,
/// #     user_id: i32,
/// #     title: String,
/// # }
/// #
/// # fn main() {
/// #     use users::dsl::*;
/// #     use posts::dsl::*;
/// #     let connection = establish_connection();
///
/// let data: Vec<(???)> = posts.inner_join(users)
///     .group_by(user_id)
///     .count()
///     .load(&connection)
///     .expect("Couldn't load query");
/// 
///  // assert_eq!(data, expected);
///
/// # }
/// ```
pub trait GroupByDsl<Expr: Expression> {
    /// Query with group by added
    type Output: Query;

    fn group_by(self, expr: Expr) -> Self::Output;
}

impl<T, Expr> GroupByDsl<Expr> for T
where
    Expr: Expression,
    T: Table + AsQuery,
    T::Query: GroupByDsl<Expr>,
{
    type Output = <T::Query as GroupByDsl<Expr>>::Output;

    fn group_by(self, expr: Expr) -> Self::Output {
        self.as_query().group_by(expr)
    }
}
