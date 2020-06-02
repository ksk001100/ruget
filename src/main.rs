mod lib;
use lib::download::parallel;
use seahorse::{color, App, Flag, FlagType};
use std::env;

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
    let args = env::args().collect();

    let app = App::new(color::red(NAME))
        .usage("ruget [url]")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .action(parallel::action)
        .flag(
            Flag::new("output", FlagType::String)
                .usage("[--output, -o]: ruget [url] --output [file name]")
                .alias("o"),
        );

    app.run(args);
}
