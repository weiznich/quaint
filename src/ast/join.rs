use crate::ast::{ConditionTree, Table};

/// The `JOIN` table and conditions.
#[derive(Debug, PartialEq, Clone)]
pub struct JoinData {
    pub table: Table,
    pub conditions: ConditionTree,
}

/// A representation of a `JOIN` statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Join {
    /// Implements an `INNER JOIN` with given `JoinData`.
    Inner(JoinData),
}

/// An item that can be joined.
pub trait Joinable {
    /// Add the `JOIN` conditions.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let join_data = "b".on(("b", "id").equals(Column::from(("a", "id"))));
    /// let query = Select::from("a").inner_join(join_data);
    /// let (sql, _) = Sqlite::build(query);
    ///
    /// assert_eq!(
    ///     "SELECT * FROM `a` INNER JOIN `b` ON `b`.`id` = `a`.`id` LIMIT -1",
    ///     sql,
    /// );
    /// ```
    fn on<T>(self, conditions: T) -> JoinData
    where
        T: Into<ConditionTree>;
}

macro_rules! joinable {
    ($($kind:ty),*) => (
        $(
            impl Joinable for $kind {
                fn on<T>(self, conditions: T) -> JoinData
                where
                    T: Into<ConditionTree>,
                {
                    JoinData {
                        table: self.into(),
                        conditions: conditions.into(),
                    }
                }
            }
        )*
    );
}

joinable!(String, (String, String));
joinable!(&str, (&str, &str));
