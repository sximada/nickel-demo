#[macro_use] extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;


use nickel::{Nickel, HttpRouter, JsonBody};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    firstname: String,
    lastname:  String,
}

fn main() {
    let mut server = Nickel::new();

    server.post("/a/post/request", middleware! { |request, response|
        let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)
    });

    server.listen("127.0.0.1:6767");
}
