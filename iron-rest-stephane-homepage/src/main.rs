extern crate iron;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

fn main() {
    println!("Server started on http://localhost:3000 !");
    Iron::new(get_form).http("0.0.0.0:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let content_type = "application/json".parse::<Mime>().unwrap();
    response.set_mut(status::Ok);
    response.set_mut(content_type);
    response.set_mut(r#"{ response: 'Hello World' }"#);

    Ok(response)
}
