use std::env;
use calamine::{open_workbook, Reader, Xlsx};

fn main() {
    // env::set_var("RUST_BACKTRACE","full");
    let wb: Xlsx<_>  = open_workbook("C:\\Users\\Administrator\\Desktop\\Test\\Test For Xlsx.xlsx").unwrap();
    // wb
}

