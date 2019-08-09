extern crate dotenv;

use std::env;
use std::collections::HashMap;
use dotenv::dotenv;
use nickel::{
    HttpRouter,
    MiddlewareResult,
    Mountable,
    Nickel,
    Request,
    Response,
    StaticFilesHandler,
};


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
    server.mount("/static/", StaticFilesHandler::new(assets_path));
    match server.listen(domain) {
        Ok(_) => (),
        Err(err) => println!("Failed: {}", err)
    }
}
