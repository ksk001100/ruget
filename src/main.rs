mod lib;

use std::{env, process::exit};

use rayon::ThreadPoolBuilder;
use seahorse::{color, Action, App, Flag, FlagType};

use lib::download_manager::DownloadManager;

const NAME: &str = "
                       _   
                      | |  
 _ __ _   _  __ _  ___| |_ 
| '__| | | |/ _` |/ _ \\ __|
| |  | |_| | (_| |  __/ |_ 
|_|   \\__,_|\\__, |\\___|\\__|
             __/ |         
            |___/";

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let args: Vec<String> = env::args().collect();
    let action: Action = |c| {
        let url = match c.args.len() {
            1 => &c.args[0],
            _ => {
                c.help();
                exit(0);
            }
        };

        let output = match c.string_flag("output") {
            Ok(o) => Some(o),
            Err(_) => None,
        };

        let download_manager = DownloadManager::new(url.to_owned(), output);
        download_manager.downloader.download();
    };

    let app = App::new(color::red(NAME))
        .usage("ruget [url]")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(
            Flag::new("output", FlagType::String)
                .description("Specify the name of the output file.")
                .alias("o"),
        );

    app.run(args);
}
