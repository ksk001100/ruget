use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::header::RANGE;
use reqwest::Client;
use std::fs::{create_dir, remove_dir_all, File};
use std::path::Path;
use std::io::{BufReader, BufWriter, Read, Write};
use std::sync::{Arc, Mutex};
use std::io::stdout;
use num_cpus;

use crate::lib::utils::{Download, get_content_length, get_file_size};

const TMP_SIZE: usize = 300000;
const TMP_DIR: &str = "ruget_tmp_dir";

pub struct ParallelDownloader {
    pub url: String,
}

impl ParallelDownloader {
    pub fn create_args(&self) -> Vec<(usize, String)> {
        let length = get_content_length(&self.url);
        let split_num = length / TMP_SIZE as i32;

        let ranges: Vec<i32> = (0..split_num).map(|n| (length + n) / split_num).collect();

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

    pub fn get_filename(&self) -> &str {
        let url_parse: Vec<&str> = self.url.split('/').collect();
        match url_parse.last() {
            Some(name) => name,
            None => panic!("cannot get file name..."),
        }
    }

    pub fn combine_files(&self, count: usize) {
        print!("\n");
        let filename = self.get_filename();
        let mut output = BufWriter::new(File::create(filename).unwrap());

        for i in 0..count {
            let mut buf: Vec<u8> = Vec::new();
            let mut file = BufReader::new(File::open(format!("{}/{}.tmp", TMP_DIR, i)).unwrap());
            file.read_to_end(&mut buf).expect("read failed...");
            output.write_all(&buf).expect("write failed...");
            print!(
                "\rWriting : [{} / {}]",
                get_file_size(((i + 1) * TMP_SIZE) as f32),
                get_file_size((count * TMP_SIZE) as f32)
            );
            stdout().flush().unwrap();
        }

        remove_dir_all(TMP_DIR).expect("remove tmp file failed...");
    }
}

impl Download for ParallelDownloader {
    fn download(&self) {
        let thread_args = self.create_args();

        println!("--- Parallel download mode ---\n");
        println!("Split count : {}", thread_args.len());
        println!("Parallel count : {}", num_cpus::get());

        let client = Client::new();
        let downloaded_count = Arc::new(Mutex::new(0));

        let total_count = thread_args.len();

        if !Path::new(TMP_DIR).exists() {
            create_dir(TMP_DIR).expect("create tmp dir failed...");
        }
        
        thread_args.into_par_iter().for_each(|arg| {
            let tmp = format!("{}/{}.tmp", TMP_DIR, arg.0);
            let mut file = File::create(tmp).unwrap();

            loop {
                let res = client
                    .get(&self.url)
                    .header(RANGE, format!("{}", arg.1))
                    .send();

                match res {
                    Ok(mut res) => {
                        if res.status().is_success() {
                            match res.copy_to(&mut file) {
                                Ok(_) => break,
                                Err(_) => continue,
                            }
                        }
                    },
                    Err(_) => continue,
                }
            };

            *downloaded_count.lock().unwrap() += 1;
            print!(
                "\rDownloading : [{} / {}]",
                get_file_size((*downloaded_count.lock().unwrap() * TMP_SIZE) as f32),
                get_file_size((total_count * TMP_SIZE) as f32)
            );
            stdout().flush().unwrap();
        });

        self.combine_files(total_count as usize);

        println!("\nDone!\n");
    }
}
