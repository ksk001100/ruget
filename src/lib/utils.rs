use reqwest::{header::ACCEPT_RANGES, Client};

pub enum RugetError {
    ClientBuiltError,
    ClientExecuteError,
}

pub fn get_file_size(b: f32) -> String {
    format!("{:.2} MB", b / 1000000.0)
}

pub fn is_accept_ranges(url: &str) -> Result<bool,RugetError> {
    let client = Client::new();

    let request_built = client.head(url).build();

    if let Err(_err) = request_built {
        return Err(RugetError::ClientBuiltError);
    }

    let res = client.execute(request_built.unwrap());

    if let Err(_err) = res {
        return Err(RugetError::ClientExecuteError);
    }

    match res.unwrap().headers().get(ACCEPT_RANGES) {
        Some(res) => Ok(!matches!(res.to_str().unwrap(), "none")),
        None => Ok(false),
    }
}

pub trait Download {
    fn download(&self);
}
