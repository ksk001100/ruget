// use std::fs::File;
// use seahorse::Context;
// use crate::lib::utils::{content_length, get_filename, get_file_size};
// use async_std::task;
//
// TODO
// pub fn action(c: &Context) {
//     task::block_on(async {
//         let uri = if c.args.len() == 1 {
//             c.args[0].to_owned()
//         } else {
//             c.help();
//             std::process::exit(0);
//         };
//
//         let fs = content_length(&uri).await.unwrap();
//     });
// }
//
// async fn downlaod(url: String) -> Result<(), surf::Exception> {
//     let res = surf::get(url).await?;
//
// }
