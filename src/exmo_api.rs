use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct ExmoAPI {
    key: String,
    secret: String,
}

impl ExmoAPI {
    pub fn new(key: String, secret: String) -> ExmoAPI {
        ExmoAPI{key, secret}
    }

    pub fn print(&self) {
        println!("EXMO API key:\t{}\nEXMO API secret:\t{}", self.key, self.secret);
    }

    pub fn load_from_file() -> ExmoAPI {
        let path = "exmoapi_key_secret.txt";
        let input = match File::open(&path) {
            Ok(file) => file,
            Err(error) => {
                panic!("There was a problem opening the file '{}', please check that '{}' exists. Error Message: {:?}", &path, &path, error)
            },
        };
        let mut reader = BufReader::new(input);

        let error_message = "
            There was a problem reading the file 'exmoapi_key_secret.txt'.
            Please check that it contains 4 lines. The length of an API key and an API
            secret is 42 characters each.\n
            Example:
            Exmo API key:
            K-****************************************
            Exmo API secret:
            S-****************************************\n";
        let mut line = String::new();
        let mut key = String::new();
        let mut secret = String::new();
        reader.read_line(&mut line).expect(error_message);
        reader.read_line(&mut key).expect(error_message);
        reader.read_line(&mut line).expect(error_message);
        reader.read_line(&mut secret).expect(error_message);
        key = key.trim().to_string();
        secret = secret.trim().to_string();
        
        let length = 42;
        let key_pref = "K-";
        let secret_pref = "S-";
        if (key.len() != length && !key.starts_with(&key_pref))
            || (secret.len() != length && !secret.starts_with(&secret_pref)) {
            panic!(error_message)
        }

        self::ExmoAPI::new(key, secret)
    }
}