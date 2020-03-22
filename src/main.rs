

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json};


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
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ScrapeResult {
    scrape_time: String,
    row_results: Vec<RowResult>,
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
fn index_bar() ->  Json<Point> {
    let point = Point { x: 1, y: 2 };
    return Json(point)
}


fn main() {
    println!("Start");
    let u = read_user_from_file().unwrap();
    println!("{:#?}", u);
    println!("Mount!");
    rocket::ignite().mount("/", routes![index, index_foo, index_bar]).launch();
    println!("Mount over!");
}

fn read_user_from_file() -> Result<ScrapeResult, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open("/Users/ksimons/code/corona/scraping/scrapedData/world/03:22:2020,18:18:35")?;
    // let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}