extern crate reqwest;
extern crate rayon;
extern crate num_cpus;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use reqwest::header::{CONTENT_LENGTH, RANGE};
use reqwest::Client;
use std::env;
use std::fs::{remove_file, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get()).build_global().unwrap();

    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let mut thread_args = create_args(url);
    let total_count = thread_args.len();
    let url_parse: Vec<&str> = url.split('/').collect();
    let file_name = match url_parse.last() {
        Some(name) => name,
        None => panic!("cannot get file name..."),
    };

    println!("split count : {}", total_count);

    download(url, &mut thread_args);
    combine_files(file_name, total_count);
}

fn download(url: &str, thread_args: &mut Vec<(usize, String)>) {
    let client = Client::new();
    let downloaded_count = Arc::new(Mutex::new(0.0));
    let total_count = thread_args.len() as f32;
    thread_args.par_iter_mut().for_each(|arg| {
        let mut res = client
            .get(url)
            .header(RANGE, format!("{}", arg.1))
            .send()
            .expect("request failed...");
        let tmp = format!("{}.tmp", arg.0);
        let path = Path::new(&tmp);
        let mut file = File::create(path).unwrap();
        res.copy_to(&mut file).expect("create failed...");
        *downloaded_count.lock().unwrap() += 1.0;
        let par = (*downloaded_count.lock().unwrap() / total_count) * 100.0;
        print!("\rDone : {:.2}%", par);
    });

    println!("\n");
}

fn combine_files(file_name: &str, file_count: usize) {
    let mut output = BufWriter::new(File::create(file_name).unwrap());

    for i in 0..file_count {
        let mut buf: Vec<u8> = Vec::new();
        let mut file = BufReader::new(File::open(format!("{}.tmp", i)).unwrap());
        file.read_to_end(&mut buf).expect("read failed...");
        output.write_all(&buf).expect("write failed...");
        remove_file(format!("{}.tmp", i)).expect("remove tmp file failed...");
    }
}

fn create_args(url: &str) -> Vec<(usize, String)> {
    let head_client = Client::new();
    let head_resp = head_client.head(url).send().expect("head failed...");

    let length = head_resp
        .headers()
        .get(CONTENT_LENGTH)
        .expect("cannot get content-length...");

    let split_num = ((length.to_str().unwrap()).parse::<i32>().unwrap()) / 300000;

    let ranges: Vec<i32> = (0..split_num)
        .map(|n| ((length.to_str().unwrap()).parse::<i32>().unwrap() + n) / split_num)
        .collect();

    (&ranges)
        .into_iter()
        .enumerate()
        .map(|(index, x)| {
            let s = match index {
                0 => 0,
                _ => (&ranges[..index]).iter().fold(0, |sum, y| sum + y) + 1,
            };
            let e = (&ranges[..index]).iter().fold(0, |sum, y| sum + y) + x;
            let range = format!("bytes={}-{}", s, e);
            (index, range)
        })
        .collect()
}