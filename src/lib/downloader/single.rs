use std::fs::File;

use reqwest;

use crate::lib::utils::Download;

pub struct SingleDownloader {
    pub url: String,
    pub output_path: Option<String>,
}

impl SingleDownloader {
    pub fn new(url: String, output_path: Option<String>) -> Self {
        Self { url, output_path }
    }

    pub fn get_filename(&self) -> &str {
        match &self.output_path {
            Some(output_path) => &output_path,
            None => {
                let url_parse: Vec<&str> = self.url.split('/').collect();
                match url_parse.last() {
                    Some(name) => name,
                    None => panic!("cannot get file name..."),
                }
            }
        }
    }
}

impl Download for SingleDownloader {
    fn download(&self) {
        println!("--- Single download mode ---\n");

        let mut res = reqwest::get(&self.url).expect("download failed...");
        let filename = self.get_filename();
        let mut file = File::create(filename).unwrap();
        res.copy_to(&mut file).expect("create failed...");
        println!("Done!\n");
    }
}
