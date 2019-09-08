use crate::lib::utils::{Download, is_accept_ranges};
use crate::lib::downloader::parallel::ParallelDownloader;
use crate::lib::downloader::single::SingleDownloader;

pub struct DownloadManager {
    pub downloader: Box<Download>
}

impl DownloadManager {
    pub fn new(url: String) -> Self {
        let downloader: Box<Download> = {
            if is_accept_ranges(&url) {
                Box::new(ParallelDownloader::new(url))
            }
            else {
                Box::new(SingleDownloader::new(url))
            }
        };
        Self { downloader }
    }
}