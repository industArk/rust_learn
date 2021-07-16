extern crate router;
use router::Router;

extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Server available at http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}


fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title>GCD Calc</title>
    <form action="/gcd" method="post">
        <input type="text" name="n" />
        <input type="text" name="n" />
        <button type="submit">Calculate GCD</button>
    </form>
    "#);

    Ok(response)
} 


fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

// in rust by convention it is OK to call libraries just before use
extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        // here we have something similar to pythons try-except
        // for creating variable form_data we check if its properly created and MATCH proper datatype
        // IF NOT we code how the error will behave, IF YES as well
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error: form data parsing error: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        // similar as above but if matched data is None then we code what will be next
        // and what will be when there is Some result (not None)
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form has no parameter 'n'\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    // create mutable variable 'numbers' of type vector by creating new vector
    let mut numbers = Vec::new();
    // for loop going over unparsed_numbers variable
    for unparsed in unparsed_numbers {
        // here we check if an iteration can be parsed to u64 type
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("'n' is not a number: {:?}\n", unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }

    let mut d = numbers[0];
    // for loop in desired range by using [1..]
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("Greatest common divisor of numbers {:?} is <b>{}</b>\n", numbers, d));

    Ok(response)
}
