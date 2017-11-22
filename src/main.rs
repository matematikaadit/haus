#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate regex;
extern crate rocket;
extern crate rusqlite;
#[macro_use] extern crate lazy_static;

use std::io;
use std::sync::Mutex;

use rocket::{Rocket, Request, State};
use rocket::request::Form;
use rocket::response::{Redirect, NamedFile};
use rocket::response::status::Created;
use rocket::fairing::AdHoc;
use rusqlite::{Connection};

mod id;
mod url;

use id::Id;


type Db = Mutex<Connection>;

struct Config {
    address: String,
    id_length: usize,
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
fn create(url: Form<url::Data>, db: State<Db>, config: State<Config>)
          -> Result<Created<String>, &'static str> {
    use rusqlite::Error::*;

    let id = id::Id::new(config.id_length);
    let conn = db.lock().expect("db connection lock");
    let url = url.get();
    match get_url(&id, &conn) {
        // no previous url saved with this id, create it
        Err(QueryReturnedNoRows) => {
            insert_url(&id, &url, &conn)
                .map(|()| {
                    Created(format!("{address}/{id}",
                                    address=config.address, id=id),
                            Some(format!("{url}", url=url)))
                })
                .map_err(|_| "cannot create id")
        }
        _ => Err("id already exist")
    }
}


#[post("/<_id>", data="<_url>")]
fn create_with_id(_id: Id, _url: Form<url::Data>) -> String {
    format!("another unimplemented")
}


#[error(404)]
fn not_found(req: &Request) -> String {
    format!("No url found for {}", req.uri())
}


fn init_config(rocket: Rocket) -> Result<Rocket, Rocket> {
    // get db_file config, use in memory database if it's not available
    let conn = match rocket.config().get_str("db_file") {
        Ok(db_file) => {
            Connection::open(db_file)
                .expect("sqlite3 database file")
        },
        _ => {
            let conn = Connection::open_in_memory()
                .expect("in memory db");
            init_db(&conn);
            conn
        }
    };

    // get other config setup
    let address = rocket.config()
        .get_str("display_address")
        .unwrap_or("http://localhost:8000")
        .to_string();
    let id_length = rocket.config()
        .get_int("id_length")
        .unwrap_or(5) as usize;

    Ok(rocket
       .manage(Mutex::new(conn))
       .manage(Config { address, id_length }))
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_attach(init_config))
        .mount("/", routes![index, retrieve, create, create_with_id])
        .catch(errors![not_found])
        .launch();
}
