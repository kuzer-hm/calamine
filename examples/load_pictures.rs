use std::env;
use calamine::{open_workbook, Reader, Xlsx};

fn main() {
    // env::set_var("RUST_BACKTRACE","full");
    let wb: Xlsx<_>  = open_workbook("F:/gns_works/QC/PO3013/QC Report for PO3013-0240822-JK.xlsx").unwrap();
    // wb
}

