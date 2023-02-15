use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
struct Author {
    id: i32,
    name: String,
    country: String
}

fn main() -> Result<(), Error> {
    let mut conn = Client::connect("host=localhost user=postgres password=postgres", NoTls)?;

    conn.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    conn.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            id: 0,
            name: key.to_string(),
            country: value.to_string()
        };

        conn.execute("INSERT INTO author (name, country) VALUES ($1, $2)",
                 &[&author.name, &author.country])?;
    }

    for row in &conn.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
