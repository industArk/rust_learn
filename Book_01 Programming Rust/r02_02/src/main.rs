// this is the wa for referencing to external crates
// #[macro_use] points that we're going to use macros from the crate
extern crate iron;
#[macro_use] extern crate mime;

// by a convention we point out what modules/functions we'll use from a crate/library
// but if it's prelude module, which has all main basic functions, we type * <all>
use iron::prelude::*;
use iron::status;

fn main() {
    println!("Access server at http://localhost:3000");
    // here below is defined server working function - all requests will go to typed function
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title>GDC Calc</title>
    <form action="/gcd" method="post">
        <input type="text" name="n" />
        <input type="text" name="n" />
        <button type="submit">Calculate GDC</button>
    </form>
    "#);
}