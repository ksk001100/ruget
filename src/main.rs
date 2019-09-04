extern crate reqwest;
extern crate rayon;
extern crate num_cpus;

use rayon::iter::{ParallelIterator, IntoParallelIterator};
use reqwest::header::{CONTENT_LENGTH, RANGE};
use reqwest::Client;
use std::env;
use std::fs::{remove_dir_all, create_dir, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::sync::{Arc, Mutex};
use std::panic;

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get() * 10).build_global().unwrap();
    panic::set_hook(Box::new(|_| {
        eprintln!("download failed...");
        remove_dir_all("ruget_tmp_dir");
    }));

    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let content_length = get_content_length(url);

    if content_length < 1000000 {
        single_download(url);
    }
    else {
        let mut thread_args = create_args(url);
        parallel_download(url, &mut thread_args);
    }
}

fn parallel_download(url: &str, thread_args: &mut Vec<(usize, String)>) {
    println!("--- Parallel download mode ---\n");
    println!("split count : {}", thread_args.len());
    println!("parallel count : {}", num_cpus::get() * 10);

    let client = Client::new();
    let downloaded_count = Arc::new(Mutex::new(0.0));
    let total_count = thread_args.len() as f32;
    let filename = filename_from_url(url);
    create_dir("ruget_tmp_dir").expect("create tmp dir failed...");
    thread_args.into_par_iter().for_each(|arg| {
        let mut res = client
            .get(url)
            .header(RANGE, format!("{}", arg.1))
            .send()
            .expect("request failed...");
        let tmp = format!("ruget_tmp_dir/{}.tmp", arg.0);
        let mut file = File::create(tmp).unwrap();
        res.copy_to(&mut file).expect("create failed...");
        *downloaded_count.lock().unwrap() += 1.0;
        let par = (*downloaded_count.lock().unwrap() / total_count) * 100.0;
        print!("\rProgress : {:.2}%", par);
    });

    combine_files(filename, total_count as usize);

    println!("\nDone!\n");
}

fn filename_from_url(url: &str) -> &str {
    let url_parse: Vec<&str> = url.split('/').collect();
    match url_parse.last() {
        Some(name) => name,
        None => panic!("cannot get file name..."),
    }
}

fn single_download(url: &str) {
    println!("--- Single download mode ---\n");

    let mut res = reqwest::get(url).expect("download failed...");
    let filename = filename_from_url(url);
    let mut file = File::create(filename).unwrap();
    res.copy_to(&mut file).expect("create failed...");
    println!("Done!\n");
}

fn combine_files(filename: &str, file_count: usize) {
    let mut output = BufWriter::new(File::create(filename).unwrap());

    for i in 0..file_count {
        let mut buf: Vec<u8> = Vec::new();
        let mut file = BufReader::new(File::open(format!("ruget_tmp_dir/{}.tmp", i)).unwrap());
        file.read_to_end(&mut buf).expect("read failed...");
        output.write_all(&buf).expect("write failed...");
    }
    remove_dir_all("ruget_tmp_dir").expect("remove tmp file failed...");
}

fn get_content_length(url: &str) -> i32 {
    let head_client = Client::new();
    let head_resp = head_client.head(url).send().expect("head failed...");

    let length = head_resp
        .headers()
        .get(CONTENT_LENGTH)
        .expect("cannot get content-length...");

    (length.to_str().unwrap()).parse::<i32>().unwrap()
}

fn create_args(url: &str) -> Vec<(usize, String)> {
    let length = get_content_length(url);
    let split_num = length / 300000;

    let ranges: Vec<i32> = (0..split_num)
        .map(|n| (length + n) / split_num)
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