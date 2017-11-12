#![feature(plugin)]
#![plugin(rocket_codegen)]


#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Json;

mod search;
use search::get_matches;

// Test using: curl -d "hello" -X POST http://localhost:8000/search
#[post("/search", data = "<pattern>")]
fn search(pattern: String) -> Json<String>  {
    if pattern.len() > 1 {
       return Json(serde_json::to_string(&get_matches(&pattern)).unwrap());
    }
    else {
        return Json("{\"status\": \"too short\"}".to_string());
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

// to properly have static files
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, search])
}

fn main() {
    rocket().launch();
}
