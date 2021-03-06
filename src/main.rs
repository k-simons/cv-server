

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json};
use rocket_contrib::serve::StaticFiles;
use rocket_cors::Cors;
use std::collections::HashMap;

use rocket_cors::CorsOptions;



use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/foo")]
fn index_foo() -> &'static str {
    "Hello, foo!"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ScrapeResult {
    usa: HashMap<String, Vec<RowResult>>,
    world: HashMap<String, Vec<RowResult>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RowResult {
    state: String,
    total: i64,
    new_cases: i64,
    total_deaths: i64,
    new_deaths: i64,
    total_recovered: i64,
    active_cases: i64,
}

#[get("/barJson", format = "json")]
fn index_bar() ->  Json<ScrapeResult> {
    let usa = read_user_from_file("/Users/ksimons/code/corona/cv-scraping/aggregatedData/usa/data.json").unwrap();
    let world = read_user_from_file("/Users/ksimons/code/corona/cv-scraping/aggregatedData/world/data.json").unwrap();
    let scrape_result = ScrapeResult { usa: usa, world: world };
    return Json(scrape_result)
}

fn make_cors() -> Cors {
    CorsOptions { // 5.
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn main() {
    println!("Start");
    println!("Mount!");
    rocket::ignite()
        .mount("/", routes![index, index_foo, index_bar])
        .mount("/public", StaticFiles::from("static"))
        .attach(make_cors())
        .launch();
    println!("Mount over!");
}

fn read_user_from_file(file: &str) -> Result<HashMap<String, Vec<RowResult>>, Box<Error>> {
    // Open the file in read-only mode with buffer.

    let file = File::open(file)?;
    // let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}