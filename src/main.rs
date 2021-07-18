#![allow(warnings)]

use std::fs::{File, read, read_to_string};
use std::io::{BufReader, BufRead, Read, Write};

use calamine::*;

use std::path::PathBuf;
pub mod mygui;
pub mod adobeCvs;



fn main() {

    let (my_csv,my_excel, my_directory) =  adobeCvs::readPaths();
    let mut file = File::open(my_csv).unwrap();
    //read cvs
    let reader = BufReader::new(file);
    let content: Vec<String> = reader.lines().map(|line|
        line.unwrap().parse::<String>().unwrap()).collect();

    //read excel
    let mut workbook: Xlsx<_> = calamine::open_workbook(my_excel).unwrap();
    let mut range= workbook.worksheet_range("Arkusz1").unwrap().unwrap();
    adobeCvs::createPDF(&content,&range, &my_directory);

}



