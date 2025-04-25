use std::env;

pub struct ArgumentData {
    pub api_key: String,
    pub file_path: String
}

impl ArgumentData {
    pub fn init() -> Self
    {
        let mut args = env::args();
        if args.len() < 3 {
            println!("usage: {} <api_key> <file_path>", args.next().unwrap());
            panic!("not enough arguments provided");
        }
        let _ = args.next().unwrap();
        ArgumentData { api_key: args.next().unwrap(), file_path: args.next().unwrap() }
    }
}