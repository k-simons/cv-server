

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json, JsonValue};


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

#[get("/barJson", format = "json")]
fn index_bar() ->  Json<Point> {
    let point = Point { x: 1, y: 2 };
    return Json(point)
}


fn main() {
    println!("Mount!");
    rocket::ignite().mount("/", routes![index, index_foo, index_bar]).launch();
    println!("Mount over!");
}
