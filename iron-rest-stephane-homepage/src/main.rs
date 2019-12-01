/******************************************************************************
 * This is a part of www.stephane-bressani.ch backend with rust
 *
 * By Stéphane Bressani
 *  ____  _             _
 * / ___|| |_ ___ _ __ | |__   __ _ _ __   ___
 * \___ \| __/ _ \ '_ \| '_ \ / _` | '_ \ / _ \
 *  ___) | ||  __/ |_) | | | | (_| | | | |  __/
 * |____/ \__\___| .__/|_| |_|\__,_|_| |_|\___|
 *               | |stephane-bressani.ch
 *               |_|github.com/stephaneworkspace
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, see <http://www.gnu.org/licenses/>.
 *****************************************************************************/
extern crate filter_city;
extern crate iron;
extern crate params;
extern crate router;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use router::Router;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    error: String,
}

fn main() {
    println!("Server started on http://localhost:3000 !");
    let mut router = Router::new();
    router.get("/city", get_form_city, "city");
    Iron::new(router).http("0.0.0.0:3000").unwrap();
}

/// This route is a filter for find a city all over the world in ASCII
fn get_form_city(_request: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    let map = _request.get_ref::<Params>().unwrap();
    let mut response = Response::new();
    let content_type = "application/json".parse::<Mime>().unwrap();
    response.set_mut(status::Ok);
    response.set_mut(content_type);
    match map.find(&["filter"]) {
        Some(&Value::String(ref filter)) => {
            let result: Vec<filter_city::City> =
                filter_city::filter_city(filter);
            let json = serde_json::to_string(&result).unwrap();
            response.set_mut(json);
        }
        _ => {
            let err: Error = Error {
                error: "Query param filter needed".to_string(),
            };
            let json = serde_json::to_string(&err).unwrap();
            response.set_mut(json);
            response.set_mut(status::BadRequest);
        }
    }
    Ok(response)
}
