use std::env;
use calamine::{open_workbook, Reader, Xlsx};

fn main() {
    // env::set_var("RUST_BACKTRACE","full");
    // env::set_var("RUST_BACKTRACE", "1");
    println!("{:?}",env::args().nth(1)) ;
    // let wb: Xlsx<_>  = open_workbook("../tests/picture.xlsx").unwrap();
    //
    // if let Some(drawings) = wb.drawings() {
    //     println!("ssss");
    // }

    // wb
}

