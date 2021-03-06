mod error;

use super::test_api::*;
use crate::{
    ast::*,
    connector::{Queryable, TransactionCapable},
};
use test_macros::test_each_connector;

#[test_each_connector]
async fn single_value(api: &mut dyn TestApi) -> crate::Result<()> {
    let select = Select::default().value("foo");
    let res = api.conn().select(select).await?.into_single()?;

    assert_eq!(Value::text("foo"), res[0]);

    Ok(())
}

#[test_each_connector]
async fn aliased_value(api: &mut dyn TestApi) -> crate::Result<()> {
    let select = Select::default().value(val!("foo").alias("bar"));
    let res = api.conn().select(select).await?.into_single()?;

    assert_eq!(Value::text("foo"), res["bar"]);

    Ok(())
}

#[test_each_connector]
async fn aliased_null(api: &mut dyn TestApi) -> crate::Result<()> {
    let query = Select::default().value(val!(Value::Integer(None)).alias("test"));

    let res = api.conn().select(query).await?;
    let row = res.get(0).unwrap();

    // No results expected.
    assert!(row["test"].is_null());

    Ok(())
}

#[test_each_connector]
async fn select_star_from(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, value int").await?;

    let insert = Insert::single_into(&table).value("value", 3).value("id", 4);
    api.conn().execute(insert.into()).await?;

    let select = Select::from_table(&table);
    let row = api.conn().select(select).await?.into_single()?;

    assert_eq!(Value::integer(4), row["id"]);
    assert_eq!(Value::integer(3), row["value"]);

    Ok(())
}

#[test_each_connector]
async fn transactions(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("value int").await?;

    let tx = api.conn().start_transaction().await?;
    let insert = Insert::single_into(&table).value("value", 10);

    let rows_affected = tx.execute(insert.into()).await?;
    assert_eq!(1, rows_affected);

    let select = Select::from_table(&table).column("value");
    let res = api.conn().select(select).await?.into_single()?;

    assert_eq!(Value::integer(10), res[0]);

    tx.rollback().await?;

    let select = Select::from_table(&table).column("value");
    let res = api.conn().select(select).await?;

    assert_eq!(0, res.len());

    Ok(())
}

#[test_each_connector]
async fn in_values_singular(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, id2 int").await?;

    let insert = Insert::multi_into(&table, vec!["id", "id2"])
        .values(vec![1, 2])
        .values(vec![3, 4])
        .values(vec![5, 6]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).so_that("id".in_selection(vec![1, 3]));

    let res = api.conn().select(query).await?;
    assert_eq!(2, res.len());

    let row1 = res.get(0).unwrap();
    assert_eq!(Some(1), row1["id"].as_i64());
    assert_eq!(Some(2), row1["id2"].as_i64());

    let row2 = res.get(1).unwrap();
    assert_eq!(Some(3), row2["id"].as_i64());
    assert_eq!(Some(4), row2["id2"].as_i64());

    Ok(())
}

#[test_each_connector]
async fn not_in_values_singular(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, id2 int").await?;

    let insert = Insert::multi_into(&table, vec!["id", "id2"])
        .values(vec![1, 2])
        .values(vec![3, 4])
        .values(vec![5, 6]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).so_that("id".not_in_selection(vec![1, 3]));

    let res = api.conn().select(query).await?;
    assert_eq!(1, res.len());

    let row1 = res.get(0).unwrap();
    assert_eq!(Some(5), row1["id"].as_i64());
    assert_eq!(Some(6), row1["id2"].as_i64());

    Ok(())
}

#[test_each_connector]
async fn in_values_tuple(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, id2 int").await?;

    let insert = Insert::multi_into(&table, vec!["id", "id2"])
        .values(vec![1, 2])
        .values(vec![3, 4])
        .values(vec![5, 6]);

    api.conn().insert(insert.into()).await?;

    let query =
        Select::from_table(table).so_that(Row::from((col!("id"), col!("id2"))).in_selection(values!((1, 2), (3, 4))));

    let res = api.conn().select(query).await?;
    assert_eq!(2, res.len());

    let row1 = res.get(0).unwrap();
    assert_eq!(Some(1), row1["id"].as_i64());
    assert_eq!(Some(2), row1["id2"].as_i64());

    let row2 = res.get(1).unwrap();
    assert_eq!(Some(3), row2["id"].as_i64());
    assert_eq!(Some(4), row2["id2"].as_i64());

    Ok(())
}

#[test_each_connector]
async fn not_in_values_tuple(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, id2 int").await?;

    let insert = Insert::multi_into(&table, vec!["id", "id2"])
        .values(vec![1, 2])
        .values(vec![3, 4])
        .values(vec![5, 6]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table)
        .so_that(Row::from((col!("id"), col!("id2"))).not_in_selection(values!((1, 2), (3, 4))));

    let res = api.conn().select(query).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(5), row["id"].as_i64());
    assert_eq!(Some(6), row["id2"].as_i64());

    Ok(())
}

#[test_each_connector]
async fn order_by_ascend(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, id2 int").await?;

    let insert = Insert::multi_into(&table, vec!["id", "id2"])
        .values(vec![3, 4])
        .values(vec![1, 2])
        .values(vec![5, 6]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).order_by("id2".ascend());

    let res = api.conn().select(query).await?;
    assert_eq!(3, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some(2), row["id2"].as_i64());

    let row = res.get(1).unwrap();
    assert_eq!(Some(3), row["id"].as_i64());
    assert_eq!(Some(4), row["id2"].as_i64());

    let row = res.get(2).unwrap();
    assert_eq!(Some(5), row["id"].as_i64());
    assert_eq!(Some(6), row["id2"].as_i64());

    Ok(())
}

#[test_each_connector]
async fn order_by_descend(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, id2 int").await?;

    let insert = Insert::multi_into(&table, vec!["id", "id2"])
        .values(vec![3, 4])
        .values(vec![1, 2])
        .values(vec![5, 6]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).order_by("id2".descend());

    let res = api.conn().select(query).await?;
    assert_eq!(3, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(5), row["id"].as_i64());
    assert_eq!(Some(6), row["id2"].as_i64());

    let row = res.get(1).unwrap();
    assert_eq!(Some(3), row["id"].as_i64());
    assert_eq!(Some(4), row["id2"].as_i64());

    let row = res.get(2).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some(2), row["id2"].as_i64());

    Ok(())
}

#[test_each_connector]
async fn where_equals(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).so_that("name".equals("Naukio"));
    let res = api.conn().select(query).await?;

    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn where_like(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).so_that("name".like("auk"));
    let res = api.conn().select(query).await?;

    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn where_not_like(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).so_that("name".not_like("auk"));
    let res = api.conn().select(query).await?;

    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn inner_join(api: &mut dyn TestApi) -> crate::Result<()> {
    let table1 = api.create_table("id int, name varchar(255)").await?;
    let table2 = api.create_table("t1_id int, is_cat int").await?;

    let insert = Insert::multi_into(&table1, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Belka")]);

    api.conn().insert(insert.into()).await?;

    let insert = Insert::multi_into(&table2, vec!["t1_id", "is_cat"])
        .values(vec![Value::integer(1), Value::integer(1)])
        .values(vec![Value::integer(2), Value::integer(0)]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(&table1)
        .column((&table1, "name"))
        .column((&table2, "is_cat"))
        .inner_join(
            table2
                .as_str()
                .on((table1.as_str(), "id").equals(Column::from((&table2, "t1_id")))),
        )
        .order_by("id".ascend());

    let res = api.conn().select(query).await?;

    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Musti"), row["name"].as_str());
    assert_eq!(Some(true), row["is_cat"].as_bool());

    let row = res.get(1).unwrap();
    assert_eq!(Some("Belka"), row["name"].as_str());
    assert_eq!(Some(false), row["is_cat"].as_bool());

    Ok(())
}

#[test_each_connector]
async fn left_join(api: &mut dyn TestApi) -> crate::Result<()> {
    let table1 = api.create_table("id int, name varchar(255)").await?;
    let table2 = api.create_table("t1_id int, is_cat int").await?;

    let insert = Insert::multi_into(&table1, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Belka")]);

    api.conn().insert(insert.into()).await?;

    let insert =
        Insert::multi_into(&table2, vec!["t1_id", "is_cat"]).values(vec![Value::integer(1), Value::integer(1)]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(&table1)
        .column((&table1, "name"))
        .column((&table2, "is_cat"))
        .left_join(
            table2
                .as_str()
                .on((&table1, "id").equals(Column::from((&table2, "t1_id")))),
        )
        .order_by("id".ascend());

    let res = api.conn().select(query).await?;

    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Musti"), row["name"].as_str());
    assert_eq!(Some(true), row["is_cat"].as_bool());

    let row = res.get(1).unwrap();
    assert_eq!(Some("Belka"), row["name"].as_str());
    assert_eq!(None, row["is_cat"].as_bool());

    Ok(())
}

#[test_each_connector]
async fn limit_no_offset(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(&table).order_by("id".descend()).limit(1);

    let res = api.conn().select(query).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();

    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn offset_no_limit(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).order_by("id".descend()).offset(1);

    let res = api.conn().select(query).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();

    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn limit_with_offset(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")])
        .values(vec![Value::integer(3), Value::text("Belka")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).order_by("id".ascend()).limit(1).offset(2);

    let res = api.conn().select(query).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();

    assert_eq!(Some("Belka"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn limit_with_offset_no_given_order(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::multi_into(&table, vec!["id", "name"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")])
        .values(vec![Value::integer(3), Value::text("Belka")]);

    api.conn().insert(insert.into()).await?;

    let query = Select::from_table(table).limit(1).offset(2);

    let res = api.conn().select(query).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Belka"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_default_value_insert(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id int default 1, name varchar(255) default 'Musti'")
        .await?;

    let changes = api.conn().execute(Insert::single_into(&table).into()).await?;
    assert_eq!(1, changes);

    let select = Select::from_table(&table);

    let res = api.conn().select(select).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector(tags("mssql", "postgres"))]
async fn returning_insert(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("id int, name varchar(255)").await?;
    let insert = Insert::single_into(&table).value("id", 2).value("name", "Naukio");

    let res = api
        .conn()
        .insert(Insert::from(insert).returning(vec!["id", "name"]))
        .await?;

    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(2), row["id"].as_i64());
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_single_unique(api: &mut dyn TestApi) -> crate::Result<()> {
    let constraint = api.unique_constraint("id");

    let table_name = api
        .create_table(&format!("id int, name varchar(255), {}", constraint))
        .await?;

    let insert = Insert::single_into(&table_name).value("id", 1).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let table = Table::from(&table_name).add_unique_index("id");
    let cols = vec![(&table_name, "id"), (&table_name, "name")];

    let insert: Insert<'_> = Insert::multi_into(table.clone(), cols)
        .values(vec![val!(1), val!("Naukio")])
        .values(vec![val!(2), val!("Belka")])
        .into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(1, changes);

    let res = api.conn().select(Select::from_table(table)).await?;
    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    let row = res.get(1).unwrap();
    assert_eq!(Some(2), row["id"].as_i64());
    assert_eq!(Some("Belka"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_single_unique_with_default(api: &mut dyn TestApi) -> crate::Result<()> {
    let constraint = api.unique_constraint("id");

    let table_name = api
        .create_table(&format!("id int default 10, name varchar(255), {}", constraint))
        .await?;

    let insert = Insert::single_into(&table_name).value("id", 10).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let id = Column::from("id").default(10);
    let table = Table::from(&table_name).add_unique_index(id);

    let insert: Insert<'_> = Insert::single_into(table.clone()).value("name", "Naukio").into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(0, changes);

    let select = Select::from_table(table);

    let res = api.conn().select(select).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(10), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_single_unique_with_autogen_default(
    api: &mut dyn TestApi,
) -> crate::Result<()> {
    let table_name = api
        .create_table(&format!("{}, name varchar(255)", api.autogen_id("id")))
        .await?;

    let id = Column::from("id").default(DefaultValue::Generated);
    let table = Table::from(&table_name).add_unique_index(id);

    let insert: Insert<'_> = Insert::single_into(table.clone()).value("name", "Naukio").into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(1, changes);

    let select = Select::from_table(table);

    let res = api.conn().select(select).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector(tags("postgres", "mssql"))]
async fn single_insert_conflict_do_nothing_with_returning(api: &mut dyn TestApi) -> crate::Result<()> {
    let constraint = api.unique_constraint("id");

    let table_name = api
        .create_table(&format!("id int, name varchar(255), {}", constraint))
        .await?;

    let insert = Insert::single_into(&table_name).value("id", 1).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let table = Table::from(&table_name).add_unique_index("id");
    let cols = vec![(&table_name, "id"), (&table_name, "name")];

    let insert: Insert<'_> = Insert::multi_into(table.clone(), cols)
        .values(vec![val!(1), val!("Naukio")])
        .values(vec![val!(2), val!("Belka")])
        .into();

    let res = api
        .conn()
        .insert(insert.on_conflict(OnConflict::DoNothing).returning(vec!["name"]))
        .await?;

    assert_eq!(1, res.len());
    assert_eq!(1, res.columns().len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Belka"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_two_uniques(api: &mut dyn TestApi) -> crate::Result<()> {
    let id_constraint = api.unique_constraint("id");
    let name_constraint = api.unique_constraint("name");

    let table_name = api
        .create_table(&format!(
            "id int, name varchar(255), {}, {}",
            id_constraint, name_constraint
        ))
        .await?;

    let insert = Insert::single_into(&table_name).value("id", 1).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let table = Table::from(&table_name).add_unique_index("id").add_unique_index("name");

    let cols = vec![(&table_name, "id"), (&table_name, "name")];

    let insert: Insert<'_> = Insert::multi_into(table.clone(), cols)
        .values(vec![val!(1), val!("Naukio")])
        .values(vec![val!(3), val!("Musti")])
        .values(vec![val!(2), val!("Belka")])
        .into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(1, changes);

    let select = Select::from_table(table).order_by("id".ascend());

    let res = api.conn().select(select).await?;
    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    let row = res.get(1).unwrap();
    assert_eq!(Some(2), row["id"].as_i64());
    assert_eq!(Some("Belka"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_two_uniques_with_default(api: &mut dyn TestApi) -> crate::Result<()> {
    let id_constraint = api.unique_constraint("id");
    let name_constraint = api.unique_constraint("name");

    let table_name = api
        .create_table(&format!(
            "id int, name varchar(255) default 'Musti', {}, {}",
            id_constraint, name_constraint
        ))
        .await?;

    let insert = Insert::single_into(&table_name).value("id", 1).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let id = Column::from("id").table(&table_name);
    let name = Column::from("name").default("Musti").table(&table_name);

    let table = Table::from(&table_name)
        .add_unique_index(id.clone())
        .add_unique_index(name.clone());

    let insert: Insert<'_> = Insert::single_into(table.clone()).value(id, 2).into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(0, changes);

    let select = Select::from_table(table).order_by("id".ascend());

    let res = api.conn().select(select).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_compoud_unique(api: &mut dyn TestApi) -> crate::Result<()> {
    let table_name = api.create_table("id int, name varchar(255)").await?;
    api.create_index(&table_name, "id asc, name asc").await?;

    let insert = Insert::single_into(&table_name).value("id", 1).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let id = Column::from("id").table(&table_name);
    let name = Column::from("name").table(&table_name);

    let table = Table::from(&table_name).add_unique_index(vec![id.clone(), name.clone()]);

    let insert: Insert<'_> = Insert::multi_into(table.clone(), vec![id, name])
        .values(vec![val!(1), val!("Musti")])
        .values(vec![val!(1), val!("Naukio")])
        .into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(1, changes);

    let select = Select::from_table(table).order_by("id".ascend());

    let res = api.conn().select(select).await?;
    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    let row = res.get(1).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_compoud_unique_with_default(api: &mut dyn TestApi) -> crate::Result<()> {
    let table_name = api.create_table("id int, name varchar(255) default 'Musti'").await?;
    api.create_index(&table_name, "id asc, name asc").await?;

    let insert = Insert::single_into(&table_name).value("id", 1).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let id = Column::from("id").table(&table_name);
    let name = Column::from("name").table(&table_name).default("Musti");

    let table = Table::from(&table_name).add_unique_index(vec![id.clone(), name.clone()]);

    let insert: Insert<'_> = Insert::single_into(table.clone()).value(id, 1).into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(0, changes);

    let select = Select::from_table(table).order_by("id".ascend());

    let res = api.conn().select(select).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_unique_with_autogen(api: &mut dyn TestApi) -> crate::Result<()> {
    let table_name = api
        .create_table(&format!("{}, name varchar(100)", api.autogen_id("id")))
        .await?;

    let insert = Insert::single_into(&table_name).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let id = Column::from("id").table(&table_name).default(DefaultValue::Generated);
    let name = Column::from("name").table(&table_name);

    let table = Table::from(&table_name).add_unique_index(vec![id.clone(), name.clone()]);
    let insert: Insert<'_> = Insert::single_into(table.clone()).value(name, "Naukio").into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(1, changes);

    let select = Select::from_table(table).order_by("id".ascend());

    let res = api.conn().select(select).await?;
    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    let row = res.get(1).unwrap();
    assert_eq!(Some(2), row["id"].as_i64());
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn single_insert_conflict_do_nothing_compoud_unique_with_autogen_default(
    api: &mut dyn TestApi,
) -> crate::Result<()> {
    let table_name = api
        .create_table(&format!("{}, name varchar(100) default 'Musti'", api.autogen_id("id")))
        .await?;

    api.create_index(&table_name, "id asc, name asc").await?;

    let insert = Insert::single_into(&table_name).value("name", "Musti");
    api.conn().insert(insert.into()).await?;

    let id = Column::from("id").table(&table_name).default(DefaultValue::Generated);
    let name = Column::from("name").table(&table_name).default("Musti");

    let table = Table::from(&table_name).add_unique_index(vec![id.clone(), name.clone()]);

    let insert: Insert<'_> = Insert::single_into(table.clone()).value(name, "Musti").into();

    let changes = api
        .conn()
        .execute(insert.on_conflict(OnConflict::DoNothing).into())
        .await?;

    assert_eq!(1, changes);

    let select = Select::from_table(table).order_by("id".ascend());

    let res = api.conn().select(select).await?;
    assert_eq!(2, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    let row = res.get(1).unwrap();
    assert_eq!(Some(2), row["id"].as_i64());
    assert_eq!(Some("Musti"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn updates(api: &mut dyn TestApi) -> crate::Result<()> {
    let table_name = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::single_into(&table_name).value("name", "Musti").value("id", 1);
    api.conn().insert(insert.into()).await?;

    let update = Update::table(&table_name).set("name", "Naukio").so_that("id".equals(1));
    let changes = api.conn().execute(update.into()).await?;

    assert_eq!(1, changes);

    let select = Select::from_table(&table_name).order_by("id".ascend());
    let res = api.conn().select(select).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some(1), row["id"].as_i64());
    assert_eq!(Some("Naukio"), row["name"].as_str());

    Ok(())
}

#[test_each_connector]
async fn deletes(api: &mut dyn TestApi) -> crate::Result<()> {
    let table_name = api.create_table("id int, name varchar(255)").await?;

    let insert = Insert::single_into(&table_name).value("name", "Musti").value("id", 1);
    api.conn().insert(insert.into()).await?;

    let delete = Delete::from_table(&table_name).so_that("id".equals(1));
    let changes = api.conn().execute(delete.into()).await?;

    assert_eq!(1, changes);

    let select = Select::from_table(&table_name).order_by("id".ascend());
    let res = api.conn().select(select).await?;
    assert_eq!(0, res.len());

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn text_columns_with_non_utf8_encodings_can_be_queried(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id integer auto_increment primary key, value varchar(100) character set gb18030")
        .await?;

    let insert = Insert::multi_into(&table, vec!["value"])
        .values(vec!["法式咸派"])
        .values(vec!["土豆"]);

    api.conn().insert(insert.into()).await?;

    let select = Select::from_table(&table).column("value");
    let rows = api.conn().select(select).await?;

    let row = rows.get(0).unwrap();
    let res = row.get("value").unwrap().as_str();
    assert_eq!(Some("法式咸派"), res,);

    let row = rows.get(1).unwrap();
    let res = row.get("value").unwrap().as_str();
    assert_eq!(Some("土豆"), res);

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn filtering_by_json_values_does_not_work_but_does_not_crash(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id int4 auto_increment primary key, nested json not null")
        .await?;

    let insert = Insert::multi_into(&table, &["nested"])
        .values(vec!["{\"isTrue\": true}"])
        .values(vec!["{\"isTrue\": false}"]);

    api.conn().query(insert.into()).await?;

    let select = Select::from_table(&table).so_that("nested".equals("{\"isTrue\": false}"));
    let result = api.conn().query(select.into()).await?;

    assert!(result.is_empty());

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn float_columns_cast_to_f32(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id int4 auto_increment primary key, f float not null")
        .await?;

    let insert = Insert::single_into(&table).value("f", 6.4123456);
    api.conn().query(insert.into()).await?;

    let select = Select::from_table(&table).column("f");
    let row = api.conn().query(select.into()).await?.into_single()?;
    let value = row.at(0).unwrap();

    assert_eq!(Some(6.412345), value.as_f64());

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn newdecimal_conversion_is_handled_correctly(api: &mut dyn TestApi) -> crate::Result<()> {
    let select = Select::default().value(sum(Value::integer(1)).alias("theone"));
    let result = api.conn().select(select).await?;

    assert_eq!(
        Value::Real(Some("1.0".parse().unwrap())),
        result.into_single().unwrap()[0]
    );

    Ok(())
}

#[test_each_connector(tags("mysql"))]
async fn unsigned_integers_are_handled(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api
        .create_table("id int4 auto_increment primary key, big bigint unsigned")
        .await?;

    let insert = Insert::multi_into(&table, &["big"])
        .values((2,))
        .values((std::i64::MAX,));
    api.conn().insert(insert.into()).await?;

    let select = Select::from_table(&table).column("big").order_by("id");
    let roundtripped = api.conn().select(select).await?;

    let expected = &[2, std::i64::MAX];
    let actual: Vec<i64> = roundtripped
        .into_iter()
        .map(|row| row.at(0).unwrap().as_i64().unwrap())
        .collect();

    assert_eq!(actual, expected);

    Ok(())
}

#[test_each_connector(tags("mysql", "postgres"))]
async fn json_filtering_works(api: &mut dyn TestApi) -> crate::Result<()> {
    let json_type = match api.system() {
        "postgres" => "jsonb",
        _ => "json",
    };

    let table = api
        .create_table(&format!("{}, obj {}", api.autogen_id("id"), json_type))
        .await?;

    let insert = Insert::single_into(&table).value("obj", serde_json::json!({ "a": "a" }));
    let second_insert = Insert::single_into(&table).value("obj", serde_json::json!({ "a": "b" }));

    api.conn().insert(insert.into()).await?;
    api.conn().insert(second_insert.into()).await?;

    // Equals
    {
        let select = Select::from_table(&table).so_that(Column::from("obj").equals(serde_json::json!({ "a": "b" })));
        let result = api.conn().select(select).await?;

        assert_eq!(result.len(), 1);

        let row = result.into_single()?;
        assert_eq!(Some(2), row["id"].as_i64());
    }

    // Not equals
    {
        let select =
            Select::from_table(&table).so_that(Column::from("obj").not_equals(serde_json::json!({ "a": "a" })));

        let result = api.conn().query(select.into()).await?;

        assert_eq!(result.len(), 1);

        let row = result.into_single()?;
        assert_eq!(Some(2), row["id"].as_i64());
    }

    Ok(())
}

#[test_each_connector]
async fn upper_fun(api: &mut dyn TestApi) -> crate::Result<()> {
    let select = Select::default().value(upper("foo").alias("val"));
    let row = api.conn().select(select).await?.into_single()?;

    assert_eq!(Some("FOO"), row["val"].as_str());

    Ok(())
}

#[test_each_connector]
async fn lower_fun(api: &mut dyn TestApi) -> crate::Result<()> {
    let select = Select::default().value(lower("BAR").alias("val"));
    let row = api.conn().select(select).await?.into_single()?;

    assert_eq!(Some("bar"), row["val"].as_str());

    Ok(())
}

#[test_each_connector]
async fn op_test_add_one_level(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int").await?;

    let insert = Insert::single_into(&table).value("a", 1).value("b", 2);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") + col!("b"));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(3), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_add_two_levels(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int, c int").await?;

    let insert = Insert::single_into(&table).value("a", 2).value("b", 3).value("c", 2);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") + val!(col!("b") + col!("c")));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(7), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_sub_one_level(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int").await?;

    let insert = Insert::single_into(&table).value("a", 2).value("b", 1);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") - col!("b"));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(1), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_sub_three_items(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int, c int").await?;

    let insert = Insert::single_into(&table).value("a", 2).value("b", 1).value("c", 1);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") - col!("b") - col!("c"));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(0), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_sub_two_levels(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int, c int").await?;

    let insert = Insert::single_into(&table).value("a", 2).value("b", 3).value("c", 1);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") - val!(col!("b") + col!("c")));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(-2), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_mul_one_level(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int").await?;

    let insert = Insert::single_into(&table).value("a", 6);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") * col!("a"));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(36), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_mul_two_levels(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int").await?;

    let insert = Insert::single_into(&table).value("a", 6).value("b", 1);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") * (col!("a") - col!("b")));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(30), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_multiple_operations(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a int, b int").await?;

    let insert = Insert::single_into(&table).value("a", 4).value("b", 2);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") - col!("b") * col!("b"));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(0), row[0].as_i64());

    Ok(())
}

#[test_each_connector]
async fn op_test_div_one_level(api: &mut dyn TestApi) -> crate::Result<()> {
    let table = api.create_table("a real, b real").await?;

    let insert = Insert::single_into(&table).value("a", 6.0).value("b", 3.0);
    api.conn().insert(insert.into()).await?;

    let q = Select::from_table(&table).value(col!("a") / col!("b"));
    let row = api.conn().select(q).await?.into_single()?;

    assert_eq!(Some(2.0), row[0].as_f64());

    Ok(())
}

#[test_each_connector(tags("postgres"))]
async fn enum_values(api: &mut dyn TestApi) -> crate::Result<()> {
    let type_name = api.get_name();
    let create_type = format!("CREATE TYPE {} AS ENUM ('A', 'B')", &type_name);
    api.conn().raw_cmd(&create_type).await?;

    let table = api
        .create_table(&format!("id SERIAL PRIMARY KEY, value {}", &type_name))
        .await?;

    api.conn()
        .insert(Insert::single_into(&table).value("value", "A").into())
        .await?;

    api.conn()
        .insert(Insert::single_into(&table).value("value", "B").into())
        .await?;

    api.conn()
        .insert(Insert::single_into(&table).value("value", Value::Enum(None)).into())
        .await?;

    let select = Select::from_table(&table).column("value").order_by("id".ascend());
    let res = api.conn().select(select).await?;

    let row = res.get(0).unwrap();
    assert_eq!(Some(&Value::enum_variant("A")), row.at(0));

    let row = res.get(1).unwrap();
    assert_eq!(Some(&Value::enum_variant("B")), row.at(0));

    let row = res.get(2).unwrap();
    assert_eq!(Some(&Value::Enum(None)), row.at(0));

    Ok(())
}
