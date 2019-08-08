use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn root<'mw, 'conn>(_req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    res.render("templates/index.html", &data)
}


fn main() {
    let mut server = Nickel::new();

    server.get("/", root);
    server.listen("127.0.0.1:6767").unwrap();
}

