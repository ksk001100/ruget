use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;

pub fn get_content_length(url: &str) -> i32 {
    let head_client = Client::new();
    let head_resp = head_client.head(url).send().expect("head failed...");

    let length = head_resp
        .headers()
        .get(CONTENT_LENGTH)
        .expect("cannot get content-length...");

    (length.to_str().unwrap()).parse::<i32>().unwrap()
}

pub trait Download {
    fn download(&self);
}
