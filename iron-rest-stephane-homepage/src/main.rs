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
 *****************************************************************************/
// extern crate filter_city; // my first crate discountinued becose
//                           // the format final is PascalCase for c#
//                           // and it's not usefull for rust community
extern crate iron;
extern crate params;
extern crate router;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate unidecode;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use router::Router;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Read;

use unidecode::unidecode;

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

/// This route is a filter for find a city all over the world
/// in ASCII with flags b64
fn get_form_city(_request: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    let map = _request.get_ref::<Params>().unwrap();
    let mut response = Response::new();
    let content_type = "application/json".parse::<Mime>().unwrap();
    response.set_mut(status::Ok);
    response.set_mut(content_type);
    match map.find(&["filter"]) {
        Some(&Value::String(ref filter)) => {
            /*let result: Vec<filter_city::City> =
                filter_city::filter_city(filter);
            */
            let result: CityFilter = filter_city(filter);
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CityFilter {
    filter: Vec<City>,
    country: Vec<Country>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    id: i32,
    country: String,
    name: String,
    lat: f32,
    lng: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    country: String,
    flag: String,
}

pub fn filter_city(filter: &str) -> CityFilter {
    // Param
    let filter_upper_decode = unidecode(filter).to_ascii_uppercase();
    let mut compare_string;

    // File
    let mut s = String::new();
    const PATH: &str = "assets/citys_flags.json";
    let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
    file_path.push(std::env::current_dir().unwrap().as_path());
    file_path.push(PATH);
    // println!("{:?}", file_path.as_path());
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let _deserialized: CityFilter = serde_json::from_str(&s).unwrap();

    // Json generate output
    let mut city_filter: CityFilter = CityFilter {
        filter: Vec::new(),
        country: Vec::new(),
    };
    for x in &_deserialized.filter {
        if filter.len() > 0 {
            compare_string =
                unidecode(x.name.clone().as_str()).to_ascii_uppercase();
            if compare_string.contains(filter_upper_decode.as_str()) {
                city_filter.filter.push(City {
                    id: x.id.clone(),
                    country: x.country.clone(),
                    name: x.name.clone(),
                    lat: x.lat.clone(),
                    lng: x.lng.clone(),
                });

                // Check if country in city_filter.country
                let mut sw = false;
                let mut country: Country = Country {
                    country: "".to_string(),
                    flag: "".to_string(),
                };
                for y in &_deserialized.country {
                    if y.country == x.country {
                        sw = true;
                        country = Country {
                            country: y.country.clone(),
                            flag: y.flag.clone(),
                        };
                        break;
                    }
                }
                if sw {
                    city_filter.country.push(country);
                }
            }
        }
    }
    city_filter
}
