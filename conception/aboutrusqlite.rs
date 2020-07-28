use std::{env, fs, fs::File, error, path::Path};

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

#[derive(Debug)]
struct Entry {
    id: i64,
    name: String,
    typical_example: String,
    translation: String,
    category_eng: String,
    category_kor: String,
}

#[derive(Debug)]
struct Example {
    entry_id: i64,
    title: Option<String>,
    source: Option<String>,
    korean: String,
    english: String,
}

#[derive(Debug)]
struct Link {
    text: String,
    entry_id: i64,
    linked_entry_id: i64
}

fn main() -> core::result::Result<(), Box<error::Error>> {
    fs::remove_file("./result.sqlite")?;
    let conn = Connection::open("./result.sqlite")?;
    conn.is_autocommit();

    conn.execute(
        "CREATE TABLE entries (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            typical_example TEXT NOT NULL,
            translation TEXT NOT NULL,
            category_eng TEXT NOT NULL,
            category_kor TEXT NOT NULL
            );",
        NO_PARAMS,
    )?;
    conn.execute("CREATE TABLE examples (
            id INTEGER PRIMARY KEY,
            entry_id INTEGER,
            title TEXT,
            source TEXT,
            english TEXT NOT NULL,
            korean TEXT NOT NULL)"
        ,NO_PARAMS
    )?;
    conn.execute("CREATE TABLE links (
            id INTEGER PRIMARY KEY,
            entry_id INTEGER NOT NULL,
            linked_entry_id INTEGER NOT NULL,
            text TEXT NOT NULL)"
        ,NO_PARAMS
    )?;

    let path = Path::new("./scraped");

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let id = entry.path().file_stem().ok_or_else(|| core::fmt::Error)?.to_str().expect("Unable to get file path name").parse::<i64>()?;

        let file = File::open(path)?;
        let document = Document::from_read(file)?;

        let name = document.find(Attr("id", "ENTRYNAME")).next().unwrap().text();
        let typical_example = document.find(Attr("id", "TYPICALEXAMPLE")).next().unwrap().text();
        let translation = document.find(Attr("id", "TRANSLATION")).next().unwrap().text();
        let category_eng = document.find(Attr("id", "CATEGORYENGLISH")).next().unwrap().text();
        let category_kor = document.find(Attr("id", "CATEGORYKOREAN")).next().unwrap().text();

        conn.execute(
            "INSERT INTO entries (id, name, typical_example, translation, category_eng, category_kor)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            &[
                &id,
                &name as &ToSql,
                &typical_example as &ToSql,
                &translation as &ToSql,
                &category_eng as &ToSql,
                &category_kor as &ToSql,
            ],
        )?;
        let entry_id = conn.last_insert_rowid();
        println!("Handling entry #{}", entry_id);

        let tbody = document.find(Attr("id", "EXAMPLES").descendant(Name("table"))).next().unwrap();
        for (i, example) in tbody.find(Class("EXAMPLEENTRY")).enumerate() {
            let title = example.find(Class("EXAMPLETITLE")).next().unwrap().text();
            let korean = example.find(Class("EXAMPLEBODY")).next().unwrap().text();
            let english = example.find(Class("EXAMPLEENGLISH")).next().unwrap().text();

            let source = match example.find(Class("EXAMPLESOURCE")).next() {
                Some(node) => Some(node.text()),
                None => None,
            };

            conn.execute(
                "INSERT INTO examples (entry_id, title, korean, english, source)
                        VALUES (?1, ?2, ?3, ?4, ?5)",
                &[
                    &entry_id,
                    &title as &ToSql,
                    &source as &ToSql,
                    &korean as &ToSql,
                    &english as &ToSql,
                ],
            )?;
            println!("  -- inserted example");
        }

        for link in document.find(Attr("id", "ENTRYLINKS").descendant(Name("a"))) {
            let linked_entry_id = match link.attr("href") {
                Some(href) => href.chars().rev().take(2).collect::<String>().parse::<i64>()?,
                None => 0,
            };
            let text = link.text();

            conn.execute(
                "INSERT INTO links (entry_id, linked_entry_id, text)
                        VALUES (?1, ?2, ?3)",
                &[
                    &entry_id,
                    &linked_entry_id,
                    &text as &ToSql,
                ],
            )?;

            println!("  -- inserted link");
        }
    }

    Ok(())
}