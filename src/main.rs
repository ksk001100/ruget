mod lib;

use std::{env, process::exit};

use rayon::ThreadPoolBuilder;

use lib::download_manager::DownloadManager;

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() * 2)
        .build_global()
        .unwrap();

    let args: Vec<String> = env::args().collect();
    let url = match args.len() {
        2 => &args[1],
        _ => {
            eprintln!("Please specify a URL...");
            exit(1);
        }
    };

    let download_manager = DownloadManager::new(url.to_owned());
    download_manager.downloader.download();
}
