pub fn get_file_size(b: f32) -> String {
    format!("{:.2} MB", b / 1048576.0)
}

#[allow(dead_code)]
pub async fn is_accept_ranges(url: &str) -> bool {
    let head = surf::head(url);
    let range = head.header("Accept-Ranges");
    match range {
        Some(rng) => match rng {
            "none" => false,
            _ => true,
        },
        None => false,
    }
}

pub fn get_filename(url: String, path: Option<String>) -> String {
    match path {
        Some(output_path) => output_path,
        None => {
            let url_parse: Vec<&str> = url.split('/').collect();
            match url_parse.last() {
                Some(name) => name.to_string(),
                None => panic!("cannot get file name..."),
            }
        }
    }
}

pub async fn content_length(url: &str) -> Result<usize, surf::Exception> {
    let head = surf::head(url).await?;
    let length = head.header("Content-Length").expect("");
    Ok(length.parse::<usize>().unwrap())
}
