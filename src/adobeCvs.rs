#![allow(warnings)]

use std::fs::{File, read, read_to_string};
use std::io::{BufReader, BufRead, Read, Write};

use calamine::*;

use pdf_canvas::*;
use pdf_canvas::graphicsstate::Color;
use std::path::PathBuf;
use std::process::exit;


#[path = "mygui.rs"] mod mygui;


/// Using gui from mygui module reads given by user paths to data from CSV, Excel files
/// Also it reads path to target directory, where PDF file "result.pdf"  with result of switching strings
/// args: None
/// returns path to CSV file : string, Excel file (xlsx):  string and target directory: String
pub fn readPaths() -> (String,String, String)
{
    let my_paths = mygui::readPath();
    let mut my_csv = "";
    let mut my_excel = "";
    mygui::messageDirectory();
    let directory = mygui::readDirectory().unwrap();
    let my_directory = directory.to_str().unwrap();

    for index in my_paths.iter(){

        match index.extension().unwrap().to_str().unwrap()
        {
            "cvs" =>  my_csv = index.to_str().unwrap(),
            "xlsx" => my_excel = index.to_str().unwrap(),
            _ => ()
        }
    }
    if my_csv.is_empty() || my_excel.is_empty(){
        mygui::messageEmptyFiles();
    }
    (my_csv.to_string(), my_excel.to_string(),my_directory.to_string())
}


/// It draws rectangle frames in PDF "result.pdf" file. Green for switched strings
/// Red for not switch strings. Sets "Times Roman" font.
/// Start point for drawing rectangles: x =10.0, y= 218.0
/// For each new added record the distance between y values go higher by 10 ( calculated in createPDF functiion)
/// args: mutable canvas - the creator of the page settings, index &str: the string result switched/not switched
/// args: my_iter: float32 - the growing delta of distance in y , rgb: Vec<u8> - color values in RGB (green and red)
///
pub fn createCanvas(canvas: &mut Canvas, index: &str, my_iter:  &f32 , rgb: &Vec<u8> )
{
    let font = BuiltinFont::Times_Roman;
    let w = font.get_width(4.0, index) + 8.0;
    canvas.set_stroke_color(Color::rgb(rgb[0], rgb[1], rgb[2])).unwrap();
    canvas.rectangle(10.0, 218.0 - my_iter, w, 6.0).unwrap();
    canvas.left_text(12.0, 220.0 - my_iter, font, 4.0, &index.trim().replace("?", " ")).unwrap();
    canvas.stroke().unwrap();

}


/// Creates PDF document with result from compared cvs and xlsx files.
/// It switches the values from first column in CVS file with
/// values from second columns in XLSX files. IF something appears more in xlsx than in cvs file
/// it is ignored
/// args: content - data from cvs file, range - data from  xlsx file, my_directory - directory
/// where the result file result.pdf wil be written
pub fn createPDF(content: &Vec<String>,range: &Range<DataType>, my_directory: &str){

    let mut my_str = String::new();
    // indicator if the content has been switched
    let mut flag = 0u32;

    // create pdf
    let mut document = Pdf::create(&format!("{}/result.pdf",my_directory))
        .expect("Create pdf file");
    // decreasing step by which new box with result is drawn and written
    let mut my_iter = 0.0_f32;


    document.render_page(180.0, 240.0, |canvas|
        {
            for index in content.iter() {
                for row in range.rows() {
                    // compare values in first column from cvs and xlsx files
                    // then switch firsy column in cvs file with second from xlsx file
                    if index.contains(row[0].get_string().unwrap()) {
                        my_str = row[1].get_string().unwrap().to_owned() + " " + row[1].get_string().unwrap();
                        flag = 1u32;
                    }

                }
                if flag == 0u32{
                    createCanvas(canvas, index, &my_iter,  &vec![255,0,0]);

                }
                else{
                    createCanvas(canvas, &my_str, &my_iter,  &vec![0,255,0]);
                }
                my_iter = my_iter + 10.0;
                flag = 0u32;
            }
            Ok(())
        }).unwrap();
    document.finish().expect("Finish document");
    mygui::messageDone();
}