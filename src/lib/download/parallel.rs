use crate::lib::utils::{content_length, get_file_size, get_filename};
use async_std::sync::Mutex;
use async_std::task;
use async_std::fs::File;
use async_std::io::{BufWriter};
use async_std::prelude::*;
use chrono::{DateTime, Local};
use seahorse::{color, Context};
use std::fs::{create_dir, metadata, remove_dir_all};
use std::io::{stdout, Read, Write};
use std::path::Path;
use std::sync::Arc;
use surf::{self, url};

const TMP_DIR: &str = "ruget_tmp_dir";

pub fn combine_files(filename: &str, count: usize) {
    print!("\n");
    let mut output = std::io::BufWriter::new(std::fs::File::create(filename).unwrap());

    for i in 0..count {
        let mut buf: Vec<u8> = Vec::new();
        let path = format!("{}/{}.tmp", TMP_DIR, i);
        let mut file = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        file.read_to_end(&mut buf).expect("read failed...");
        output.write_all(&buf).expect("write failed...");
        print!(
            "\rWriting : [{} / {}]",
            get_file_size(((i + 1) as u64 * metadata(&path).unwrap().len()) as f32),
            get_file_size((count as u64 * metadata(&path).unwrap().len()) as f32)
        );
        stdout().flush().unwrap();
    }

    remove_dir_all(TMP_DIR).expect("remove tmp file failed...");
}

pub async fn spawn_args(url: &str) -> Result<Vec<(usize, String, usize)>, surf::Exception> {
    let content_length = content_length(url).await?;
    let split_num = std::cmp::min(150, content_length / 300000);
    let ranges: Vec<usize> = (0..split_num)
        .map(|n| (content_length + n) / split_num)
        .collect();

    Ok((&ranges)
        .iter()
        .enumerate()
        .map(|(index, x)| {
            let s = match index {
                0 => 0,
                _ => (&ranges[..index]).iter().fold(0, |sum, y| sum + *y) + 1,
            };
            let e = (&ranges[..index]).iter().fold(0, |sum, y| sum + *y) + *x;
            let range = format!("bytes={}-{}", s, e);
            (index, range, e - s)
        })
        .collect())
}

pub fn action(c: &Context) {
    task::block_on(async {
        if !Path::new(TMP_DIR).exists() {
            create_dir(TMP_DIR).expect("create tmp dir failed...");
        }

        let uri = if c.args.len() == 1 {
            c.args[0].to_owned()
        } else {
            println!("{}", color::red("Argument error..."));
            println!("{}", color::red("Please call `--help`"));
            std::process::exit(0);
        };

        let args = spawn_args(&uri).await.expect("");
        let total_count = args.len();
        let downloaded_count = Arc::new(Mutex::new(0));
        let mut tasks = Vec::new();

        for arg in args {
            let downloaded_count = downloaded_count.clone();

            let url = url::Url::parse(&uri).expect("");
            tasks.push(task::spawn(async move {
                let res = surf::get(url).set_header("Range", arg.1).recv_bytes().await.expect("");
                {
                    let tmp = format!("{}/{}.tmp", TMP_DIR, arg.0);
                    let buf = File::create(tmp).await.expect("");
                    let mut file = BufWriter::new(buf);
                    file.write_all(&res).await.expect("");
                }
                *downloaded_count.lock().await += 1;
                print!(
                    "\rDownloading : [{} / {}]",
                    get_file_size((*downloaded_count.lock().await * arg.2) as f32),
                    get_file_size((total_count * arg.2) as f32)
                );
                stdout().flush().unwrap();
            }));
        }

        for t in tasks {
            t.await;
        }

        let output = match c.string_flag("output") {
            Ok(o) => Some(o),
            Err(_) => None,
        };

        let filename = get_filename(uri, output);

        combine_files(&filename, total_count);

        let local_datetime: DateTime<Local> = Local::now();
        println!("\n{} - '{}' saved", local_datetime, &filename);
    });
}
