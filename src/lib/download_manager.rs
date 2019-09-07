use crate::lib::utils::{Download, get_content_length};
use crate::lib::downloader::parallel::ParallelDownloader;
use crate::lib::downloader::single::SingleDownloader;

pub struct DownloadManager {
    pub downloader: Box<Download>
}

impl DownloadManager {
    pub fn new(url: String) -> Self {
        let downloader: Box<Download> = {
            if get_content_length(&url) < 1000000 {
                Box::new(SingleDownloader::new(url))
            }
            else {
                Box::new(ParallelDownloader::new(url))
            }
        };
        Self { downloader }
    }
}