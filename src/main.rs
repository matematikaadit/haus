#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
extern crate rusqlite;

use std::io;
use std::env;
use std::sync::Mutex;

mod id;

use id::Id;
use rocket::State;
use rocket::response::{Redirect, NamedFile};
use rusqlite::{Connection};


type Db = Mutex<Connection>;

fn init_db(conn: &Connection) {
    conn.execute("CREATE TABLE urls (
                      id  TEXT PRIMARY KEY,
                      url TEXT NOT NULL
                  )", &[])
        .expect("create urls table");

    conn.execute("INSERT INTO urls (id, url) VALUES (?1, ?2)",
                 &[&"rust", &"https://www.rust-lang.org/"])
        .expect("insert single entry into urls table");
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<id>")]
fn retrieve(id: Id, db: State<Db>) -> rusqlite::Result<Redirect> {
    db.lock()
        .expect("db connection lock")
        .query_row("SELECT url FROM urls WHERE id = ?1",
                   &[&format!("{}", id)], |row| row.get(0))
        .map(|url: String| Redirect::found(&url))
}

fn main() {
    let mut args = env::args_os();
    args.next(); // ignore command name
    let conn = if let Some(db_name) = args.next() {
        Connection::open(db_name).expect("sqlite database file")
    } else {
        eprintln!("Using in memory database");
        let conn = Connection::open_in_memory().expect("in memory db");
        init_db(&conn);
        conn
    };

    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![index, retrieve]).launch();
}
