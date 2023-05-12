use std::{
    fs::{create_dir, remove_dir_all, File},
    io::{stdout, BufReader, BufWriter, Read, Write},
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::{
    blocking::Client,
    header::{CONTENT_LENGTH, RANGE},
};

use crate::lib::utils::{get_file_size, Download};

const TMP_SIZE: usize = 300000;
const TMP_DIR: &str = "ruget_tmp_dir";

pub struct ParallelDownloader {
    pub url: String,
    pub output_path: Option<String>,
    pub client: Client,
}

impl ParallelDownloader {
    pub fn new(url: String, output_path: Option<String>) -> Self {
        let client = Client::new();
        Self {
            url,
            output_path,
            client,
        }
    }

    pub fn create_args(&self) -> Vec<(usize, String)> {
        let content_length = self.get_content_length();
        let split_num = content_length / TMP_SIZE;
        let ranges: Vec<usize> = (0..split_num)
            .map(|n| (content_length + n) / split_num)
            .collect();

        (&ranges)
            .iter()
            .enumerate()
            .map(|(index, x)| {
                let s = match index {
                    0 => 0,
                    _ => (&ranges[..index]).iter().fold(0, |sum, y| sum + *y) + 1,
                };
                let e = (&ranges[..index]).iter().fold(0, |sum, y| sum + *y) + *x;
                let range = format!("bytes={}-{}", s, e);
                (index, range)
            })
            .collect()
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

    pub fn combine_files(&self, count: usize) {
        println!();
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

    pub fn get_content_length(&self) -> usize {
        let resp = self.client.head(&self.url).send().expect("head failed...");

        let length = resp
            .headers()
            .get(CONTENT_LENGTH)
            .expect("cannot get content-length...");

        (length.to_str().unwrap()).parse::<usize>().unwrap()
    }
}

impl Download for ParallelDownloader {
    fn download(&self) {
        let thread_args = self.create_args();

        println!("--- Parallel download mode ---\n");
        println!("Split count : {}", thread_args.len());
        println!("Parallel count : {}", num_cpus::get());

        let downloaded_count = Arc::new(AtomicUsize::new(0));
        let total_count = thread_args.len();

        if !Path::new(TMP_DIR).exists() {
            create_dir(TMP_DIR).expect("create tmp dir failed...");
        }

        thread_args.into_par_iter().for_each(|arg| {
            let downloaded_count = downloaded_count.clone();
            loop {
                let res = self
                    .client
                    .get(&self.url)
                    .header(RANGE, arg.1.to_string())
                    .send();

                match res {
                    Ok(mut res) => {
                        if res.status().is_success() {
                            let tmp = format!("{}/{}.tmp", TMP_DIR, arg.0);
                            let mut file = File::create(tmp).unwrap();
                            match res.copy_to(&mut file) {
                                Ok(_) => break,
                                Err(_) => continue,
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }

            downloaded_count.fetch_add(1, Ordering::Relaxed);
            print!(
                "\rDownloading : [{} / {}]",
                get_file_size((downloaded_count.load(Ordering::SeqCst) * TMP_SIZE) as f32),
                get_file_size((total_count * TMP_SIZE) as f32)
            );
            stdout().flush().unwrap();
        });

        self.combine_files(total_count as usize);

        println!("\nDone!\n");
    }
}
