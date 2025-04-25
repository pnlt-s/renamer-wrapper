use reqwest::{blocking::Client, StatusCode};
use sha256;
use std::{fs, io::{Read, Write}};

use serde::{Deserialize, Serialize};

mod argparse;

#[derive(Serialize, Deserialize)]
struct RenameRequest {
    pub code: String,
    pub hash: String,
    pub filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RenameResponse {
    pub outputCode: String,
    pub totalTokens: i32
}

fn main() {
    let args = argparse::ArgumentData::init();
    println!("api key: {}, file path: {}", args.api_key, args.file_path);

    let mut file = fs::File::open(&args.file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let _len = data.len();
    let hash = sha256::digest(&data);

    let req_data = RenameRequest {
        code: data,
        hash,
        filename: args.file_path.split('/').last().unwrap().to_string(),
    };

    let client = Client::new();
    let mut res = client
        .post("https://renamer.mshq.dev/api/rename")
        .header("ApiKey", &args.api_key)
        .header("User-Agent", "Oracle renamer wrapper for Rust")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&req_data).unwrap())
        .send().unwrap();

    let save_path = &args.file_path.replace(".lua", "-renamed.lua");

    match res.status() {
        StatusCode::OK => {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            println!("{}\nsaving to {}", body, save_path);
            let res_data: RenameResponse = serde_json::from_str(&body.as_str()).unwrap();
            let mut out = fs::File::create(save_path).unwrap();
            out.write_all(res_data.outputCode.as_bytes()).unwrap();
        },
        _ => println!("Unkwown Ewwow: {:}, {}", &res.status(), &res.text().unwrap())
    };
}
