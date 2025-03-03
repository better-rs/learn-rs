use rs_entity::user;
use sea_orm::*;

struct UserDao {
    conn: DatabaseConnection,
}

impl UserDao {
    fn init(&mut self, mut conn: DatabaseConnection) {
        self.conn = conn
    }

    fn get_user(&self) -> Result<user::Model, DbErr> {
        todo!()
    }

    fn add_user(&mut self) {}
}

#[cfg(test)]
mod tests {
    use sea_orm::{
        entity::{prelude::*, *},
        tests_cfg::*,
        DatabaseBackend, MockDatabase, Transaction,
        TransactionError::Transaction,
    };

    #[async_std::test]
    async fn test_find_cake() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                // First query result
                vec![cake::Model { id: 1, name: "New York Cheese".to_owned() }],
                // Second query result
                vec![
                    cake::Model { id: 1, name: "New York Cheese".to_owned() },
                    cake::Model { id: 2, name: "Chocolate Forest".to_owned() },
                ],
            ])
            .append_query_results(vec![
                // Third query result
                vec![(
                    cake::Model { id: 1, name: "Apple Cake".to_owned() },
                    fruit::Model { id: 2, name: "Apple".to_owned(), cake_id: Some(1) },
                )],
            ])
            .into_connection();

        // Find a cake from MockDatabase
        // Return the first query result
        assert_eq!(
            cake::Entity::find().one(&db).await?,
            Some(cake::Model { id: 1, name: "New York Cheese".to_owned() })
        );

        // Find all cakes from MockDatabase
        // Return the second query result
        assert_eq!(
            cake::Entity::find().all(&db).await?,
            vec![
                cake::Model { id: 1, name: "New York Cheese".to_owned() },
                cake::Model { id: 2, name: "Chocolate Forest".to_owned() },
            ]
        );

        // Find all cakes with its related fruits
        assert_eq!(
            cake::Entity::find().find_also_related(fruit::Entity).all(&db).await?,
            vec![(
                cake::Model { id: 1, name: "Apple Cake".to_owned() },
                Some(fruit::Model { id: 2, name: "Apple".to_owned(), cake_id: Some(1) })
            )]
        );

        // Checking transaction log
        assert_eq!(
            db.into_transaction_log(),
            vec![
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
                    vec![1u64.into()]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id", "cake"."name" FROM "cake""#,
                    vec![]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name", "fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id" FROM "cake" LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
                    vec![]
                ),
            ]
        );

        Ok(())
    }
}
