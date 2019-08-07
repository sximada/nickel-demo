#[macro_use] extern crate nickel;

use std::io::Write;
use nickel::status::StatusCode::NotFound;
use nickel::{Nickel, NickelError, Action, Continue, Halt, Request};

fn main() {
    let mut server = Nickel::new();

    fn custom_404<'a>(err: &mut NickelError, _req: &mut Request) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == NotFound {
                let _ = res.write_all(b"<p>Call the police!</p>");
                return Halt(())
            }
        }
        Continue(())
    }

    let custom_hander: fn(&mut NickelError, &mut Request) -> Action = custom_404;
    server.handle_error(custom_hander);
    server.listen("127.0.0.1:6767");
}
