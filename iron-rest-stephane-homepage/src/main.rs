/******************************************************************************
 * This is a part of www.stephane-bressani.ch backend with rust
 * -> https://stackoverflow.com/questions/37561593/how-can-i-use-serde-with-a-json-array-with-different-objects-for-successes-and-e
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
extern crate iron;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct City {
    country: String,
    name: String,
    lat: String,
    lng: String,
}

fn main() {
    println!("Server started on http://localhost:3000 !");
    Iron::new(get_form).http("0.0.0.0:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let content_type = "application/json".parse::<Mime>().unwrap();
    response.set_mut(status::Ok);
    response.set_mut(content_type);
    // response.set_mut(r#"{ response: 'Hello World' }"#);
    let result = filter_city("Genève");
    let json = serde_json::to_string(&result).unwrap();
    response.set_mut(json);
    Ok(response)
}

#[allow(unused_variables)]
fn filter_city(filter: &str) -> Vec<City> {
    // let mut json = String::new();
    let mut s = String::new();
    const PATH: &str = "/home/stephane/Code/Rust/ironRestApiStephaneHomePage/assets/citys.json";
    File::open(PATH).unwrap().read_to_string(&mut s).unwrap();
    let _deserialized: Vec<City> = serde_json::from_str(&s).unwrap();
    let mut city: Vec<City> = Vec::new();
    for x in &_deserialized {
        city.push(City {
            country: x.country.clone(),
            name: x.name.clone(),
            lat: x.lat.clone(),
            lng: x.lng.clone(),
        });
        // it's wanted to compute the for loop in its way
        // it's just for test with another rust program
        // json = format!("{}", x.name);
        // json = [json, x.name.clone()].concat();
        //println!("{}", x.name);
    }
    // json
    city
}
