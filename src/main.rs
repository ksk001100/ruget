mod lib;

use std::{env, process::exit};

use rayon::ThreadPoolBuilder;
use seahorse::{Action, SingleApp, color};

use lib::download_manager::DownloadManager;

const DISPLAY_NAME: &'static str = "                                                
                                               ___     
                                             ,--.'|_   
  __  ,-.         ,--,                       |  | :,'  
,' ,'/ /|       ,'_ /|  ,----._,.            :  : ' :  
'  | |' |  .--. |  | : /   /  ' /   ,---.  .;__,'  /   
|  |   ,','_ /| :  . ||   :     |  /     \\ |  |   |    
'  :  /  |  ' | |  . .|   | .\\  . /    /  |:__,'| :    
|  | '   |  | ' |  | |.   ; ';  |.    ' / |  '  : |__  
;  : |   :  | : ;  ; |'   .   . |'   ;   /|  |  | '.'| 
|  , ;   '  :  `--'   \\`---`-'| |'   |  / |  ;  :    ; 
 ---'    :  ,      .-./.'__/\\_: ||   :    |  |  ,   /  
          `--`----'    |   :    : \\   \\  /    ---`-'   
                        \\   \\  /   `----'              
                         `--`-'                        
";

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() * 2)
        .build_global()
        .unwrap();

    let args: Vec<String> = env::args().collect();
    let action: Action = |v: Vec<String>| {
        let url = match v.len() {
            1 => &v[0],
            _ => {
                eprintln!("Please specify a URL...");
                exit(1);
            }
        };

        let download_manager = DownloadManager::new(url.to_owned());
        download_manager.downloader.download();
    };

    let app = SingleApp::new()
        .name("ruget")
        .display_name(color::red(DISPLAY_NAME))
        .usage("ruget [url]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action);

    app.run(args);
}