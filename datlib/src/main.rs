use datlib::{as_hex, get_header, load};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Loading {}...", file_path);

    let contents = load(file_path);

    // println!("Contents:");
    // for (i, chunk) in contents.chunks(16).enumerate() {
    //     let index = format!("{:0>8}", i);
    //     println!("{}:\t{}", index, as_hex(chunk.to_vec()).join("  "));
    // }

    let expected = contents.len();
    let result = get_header(contents);
    println!("file_size: {:#?}", result.file_size);
    println!("contents.len: {}", expected);
}
