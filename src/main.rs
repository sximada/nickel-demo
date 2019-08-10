extern crate dotenv;

use std::env;
use std::collections::HashMap;

use dotenv::dotenv;
use nickel::{
    HttpRouter,
    JsonBody,
    MiddlewareResult,
    Mountable,
    Nickel,
    Request,
    Response,
    StaticFilesHandler,
};
use nickel::status::StatusCode;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Person {
    firstname: String,
    lastname:  String,
}



fn api_handler<'mw, 'conn>(req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let person = req.json_as::<Person>().unwrap();
    res.send(serde_json::to_value(person).map_err(|e| (StatusCode::InternalServerError, e)))
    // res.send(format!("Hello {} {}", person.firstname, person.lastname))
}



fn root<'mw, 'conn>(_req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    res.render("templates/index.html", &data)
}


fn main() {
    dotenv().ok();

    let mut server = Nickel::new();
    let domain = match env::var("HTTP_DOMAIN") {
        Ok(host) => host,
        Err(_) => "0.0.0.0:6767".to_string(),
    };
    let assets_path = match env::var("ASSETS_PATH") {
        Ok(path) => path,
        Err(_) => "assets".to_string(),
    };

    server.get("/", root);
    server.post("/api/", api_handler);
    server.mount("/static/", StaticFilesHandler::new(assets_path));
    match server.listen(domain) {
        Ok(_) => (),
        Err(err) => println!("Failed: {}", err)
    }
}
