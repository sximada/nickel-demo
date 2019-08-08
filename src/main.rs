use std::collections::HashMap;
use nickel::{
    HttpRouter,
    MiddlewareResult,
    Nickel,
    Request,
    Response,
    StaticFilesHandler,
};
use nickel::Mountable;

fn root<'mw, 'conn>(_req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    res.render("templates/index.html", &data)
}


fn main() {
    let mut server = Nickel::new();
    server.get("/", root);
    server.mount("/static/", StaticFilesHandler::new("assets"));
    server.listen("127.0.0.1:6767").unwrap();
}

