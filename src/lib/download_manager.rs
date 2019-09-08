use crate::lib::{
    downloader::{parallel::ParallelDownloader, single::SingleDownloader},
    utils::{is_accept_ranges, Download},
};

pub struct DownloadManager {
    pub downloader: Box<dyn Download>,
}

impl DownloadManager {
    pub fn new(url: String) -> Self {
        let downloader: Box<dyn Download> = {
            if is_accept_ranges(&url) {
                Box::new(ParallelDownloader::new(url))
            } else {
                Box::new(SingleDownloader::new(url))
            }
        };
        Self { downloader }
    }
}
