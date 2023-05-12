use crate::lib::{
    downloader::{parallel::ParallelDownloader, single::SingleDownloader},
    utils::{is_accept_ranges, Download, RugetError},
};

pub struct DownloadManager {
    pub downloader: Box<dyn Download>,
}

impl DownloadManager {
    pub fn new(url: String, output_path: Option<String>) -> Result<Self,RugetError> {

        let downloader: Box<dyn Download> = {
            if is_accept_ranges(&url)? {
                Box::new(ParallelDownloader::new(url, output_path))
            } else {
                Box::new(SingleDownloader::new(url, output_path))
            }
        };
        Ok( Self { downloader } )
    }
}
