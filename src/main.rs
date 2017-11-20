#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate regex;
extern crate rocket;
extern crate rusqlite;
#[macro_use] extern crate lazy_static;

use std::io;
use std::fmt;
use std::env;
use std::sync::Mutex;

mod id;
mod url;

use id::Id;
use rocket::{State, Request};
use rocket::request::Form;
use rocket::response::{Redirect, NamedFile};
use rocket::response::status::Created;
use rocket::fairing::AdHoc;
use rusqlite::{Connection};

const ID_LENGTH: usize = 3;
type Db = Mutex<Connection>;

struct Address(String);


impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


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


fn get_url(id: &Id, conn: &Connection) -> rusqlite::Result<String> {
    conn.query_row("SELECT url FROM urls WHERE id = ?1",
                   &[&id.as_ref()], |row| row.get(0))
}


fn insert_url(id: &Id, url: &url::Data, conn: &Connection)
              -> rusqlite::Result<()> {
    conn.execute("INSERT INTO urls (id, url) VALUES (?1, ?2)",
                 &[&id.as_ref(), &url.as_ref()])
        .map(|_| ())

}


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}


#[get("/<id>")]
fn retrieve(id: Id, db: State<Db>) -> Option<Redirect> {
    let conn = db.lock().expect("db connection lock");
    get_url(&id, &conn).map(|url| Redirect::found(&url)).ok()
}


#[post("/", data = "<url>")]
fn create(url: Form<url::Data>, db: State<Db>, address: State<Address>)
          -> Result<Created<String>, &'static str> {
    use rusqlite::Error::*;

    let id = id::Id::new(ID_LENGTH);
    let conn = db.lock().expect("db connection lock");
    let url = url.get();
    match get_url(&id, &conn) {
        // no previous url saved with this id, create it
        Err(QueryReturnedNoRows) => {
            insert_url(&id, &url, &conn)
                .map(|()| {
                    Created(format!("{address}/{id}", address=&*address, id=id),
                            Some(format!("{url}", url=url)))
                })
                .map_err(|_| "cannot create id")
        }
        _ => Err("id already exist")
    }
}


#[post("/<id>", data="<url>")]
fn create_with_id(id: Id, url: Form<url::Data>) -> String {
    format!("another unimplemented")
}


#[error(404)]
fn not_found(req: &Request) -> String {
    format!("No url found for {}", req.uri())
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
        .attach(AdHoc::on_attach(|rocket| {
            let address = rocket.config()
                .get_str("display_address")
                .unwrap_or("http://localhost:8000")
                .to_string();
            Ok(rocket.manage(Address(address)))
        }))
        .manage(Mutex::new(conn))
        .mount("/", routes![index, retrieve, create, create_with_id])
        .catch(errors![not_found])
        .launch();
}
