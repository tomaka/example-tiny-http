use postgres::Connection;

/// Ensures that the database has the correct schema.
pub fn migrate(connection: &Connection) {
    connection.execute(r#"
        CREATE TABLE IF NOT EXISTS migrations (
            id VARCHAR(64) PRIMARY KEY NOT NULL
        )
    "#, &[]).unwrap();

    // getting list of migrations already performed in the past
    let existing_migrations = connection.prepare(r#"
            SELECT id FROM migrations
        "#).unwrap();
    let existing_migrations = existing_migrations.query(&[]).unwrap();
    let existing_migrations = existing_migrations.into_iter().map(|r| r.get(0))
                                                 .collect::<Vec<String>>();

    for Migration { id, query } in get_migrations() {
        if existing_migrations.iter().find(|i| i == &id).is_some() {
            continue;
        }

        connection.execute(query, &[]).unwrap();
        connection.execute("INSERT INTO migrations(id) VALUES ($1)", &[&id]).unwrap();
    }
}

struct Migration {
    id: &'static str,
    query: &'static str,
}

fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            id: "0",
            query: r#"
                CREATE TABLE users (
                    id SERIAL PRIMARY KEY,
                    login VARCHAR(128),
                    password VARCHAR(128)
                )
            "#,
        }
    ]
}
