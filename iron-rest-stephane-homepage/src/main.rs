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
use std::io::Write;

use unidecode::unidecode;

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    error: String,
}

fn main() {
    println!("Generate asset...");
    generate();
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
            /*let result: Vec<filter_city::City> =
                filter_city::filter_city(filter);
            */
            let result: Vec<City> = filter_city(filter);
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    Id: i32,
    Name: String,
    Lat: f32,
    Lng: f32,
    Country: String,
    Flag: String,
}

pub fn filter_city(filter: &str) -> Vec<City> {
    let filter_upper_decode = unidecode(filter).to_ascii_uppercase();
    let mut compare_string;
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
    let _deserialized: Vec<City> = serde_json::from_str(&s).unwrap();
    let mut city: Vec<City> = Vec::new();
    for x in &_deserialized {
        if filter.len() > 0 {
            compare_string =
                unidecode(x.Name.clone().as_str()).to_ascii_uppercase();
            if compare_string.contains(filter_upper_decode.as_str()) {
                city.push(City {
                    Id: x.Id.clone(),
                    Country: x.Country.clone(),
                    Name: x.Name.clone(),
                    Lat: x.Lat.clone(),
                    Lng: x.Lng.clone(),
                    Flag: x.Flag.clone(),
                });
            }
        }
    }
    city
}

#[derive(Serialize, Deserialize, Debug)]
struct CityInitial {
    country: String,
    name: String,
    lat: String,
    lng: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Flag {
    id: i32,
    name: String,
    isoAlpha2: String,
    isoAlpha3: String,
    isoNumeric: i32,
    currency: Currency,
    flag: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Currency {}

fn generate() {
    const PATH_CITY: &str = "assets/citys.json";
    let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
    file_path.push(std::env::current_dir().unwrap().as_path());
    file_path.push(PATH_CITY);
    let mut s = String::new();
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let _deserialized: Vec<CityInitial> = serde_json::from_str(&s).unwrap();

    const PATH_FLAG: &str = "assets/flags.json";
    let mut file_path_flag: std::path::PathBuf = std::path::PathBuf::new();
    file_path_flag.push(std::env::current_dir().unwrap().as_path());
    file_path_flag.push(PATH_FLAG);
    let mut f = String::new();
    File::open(file_path_flag.as_path())
        .unwrap()
        .read_to_string(&mut f)
        .unwrap();
    let _deserialized_flag: Vec<Flag> = serde_json::from_str(&f).unwrap();

    let mut city_final: Vec<City> = Vec::new();
    let mut i: i32 = 0;
    let mut flag: String;
    for x in &_deserialized {
        i += 1;
        flag = "".to_string();
        for y in &_deserialized_flag {
            if y.isoAlpha2 == x.country.clone() {
                flag = y.flag.clone();
                break;
            }
        }
        city_final.push(City {
            Id: i,
            Name: x.name.clone(),
            Lat: x.lat.clone().parse().unwrap(),
            Lng: x.lng.clone().parse().unwrap(),
            Country: x.country.clone(),
            Flag: flag.clone(),
        });
    }
    let _serialized: String = serde_json::to_string(&city_final).unwrap();
    const PATH_WRITE: &str = "assets/citys_flags.json";
    let mut file_path_write: std::path::PathBuf = std::path::PathBuf::new();
    file_path_write.push(std::env::current_dir().unwrap().as_path());
    file_path_write.push(PATH_WRITE);
    let mut buffer = File::create(file_path_write.as_path()).unwrap();
    buffer.write_all(_serialized.as_bytes()).unwrap();
    buffer.flush().unwrap();
}
