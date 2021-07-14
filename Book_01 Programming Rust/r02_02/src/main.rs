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


// &mut is a mutable but referenced from other variable
// _ in _request says to the rustc "I know it's not used, don't panic!"
// Result has to point at same type as function's return data
// Result points at Ok or Err, accordingto the return that is given from function
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // sets HTTP response status to 200 OK
    response.set_mut(status::Ok);
    // sets response type - here to HTML document
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // this way is possible to just write HTML code
    // r#"something"# allows having raw string - # can be multiple, always one more that possible to find inside the string
    // such raw string may have all possible signs inside without escaping them
    response.set_mut(r#"
    <title>GDC Calc</title>
    <form action="/gcd" method="post">
        <input type="text" name="n" />
        <input type="text" name="n" />
        <button type="submit">Calculate GDC</button>
    </form>
    "#);

    Ok(response)
} 