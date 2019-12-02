extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct City {
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct CityFinal {
    Id: i32,
    Name: String,
    Lat: f32,
    Lng: f32,
    Country: String,
    Flag: String,
}

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
    let _deserialized: Vec<City> = serde_json::from_str(&s).unwrap();

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

    let mut city_final: Vec<CityFinal> = Vec::new();
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
        city_final.push(CityFinal {
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
