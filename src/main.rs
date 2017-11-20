#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;

use std::io;
use std::env;
use std::sync::Mutex;

use rocket::response::NamedFile;
use rusqlite::{Connection, Error};

type Db = Mutex<Connection>;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn main() {
    let mut args = env::args_os();
    args.next(); // ignore command name
    let conn = if let Some(db_name) = args.next() {
        Connection::open(db_name).expect("sqlite database file")
    } else {
        eprintln!("Using in memory database");
        Connection::open_in_memory().expect("in memory db")
    };

    rocket::ignite().mount("/", routes![index]).launch();
}
