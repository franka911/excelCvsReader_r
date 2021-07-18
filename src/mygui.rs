use native_dialog::{FileDialog, MessageDialog, MessageType, Error};
use std::path;
use std::env;
use std::path::PathBuf;
use std::process::exit;
extern crate exitcode;

/// GUI for reading cvs, xlsx files and the drectory to store result - result.pdf
/// the result is vector of realtive paths to files.
/// args: None
pub fn readPath() -> Vec<PathBuf>{
    let mut mypath:Vec<PathBuf> = Vec::new();
    while(true){
        // open first dialog window with current directory of projecy to choose cvs and xlsx files
        // optins with filtering system files - all files visible, onl cvs, txt and xlsx
        mypath = FileDialog::new().set_location(&env::current_dir().unwrap())
            .add_filter("All", &["*"])
            .add_filter("TXT files", &["csv", "txt", "xlsx"]).show_open_multiple_file().unwrap();

        // if cancel button is clicked on the dialog window
        // then the program stops
        if mypath.is_empty(){
            messageCancel();
            exit(exitcode::NOINPUT);
        }

        // check if there are less than two files selected
        if mypath.len() < 2 {
            let yes = MessageDialog::new().set_type(MessageType::Info).set_text("Give 2 files: PDF, CVS").show_confirm().unwrap();
            // yes button - another try to select good files
            if yes
            {
                mypath.clear();
            }
            else {
                //no button -exit program
                exit(exitcode::OK); //windows
            }
        }
        else {
            // select 2 files
            // check if there is other files selected than cvs or xlsx files
            // if yes terminate program
            if checkFilesExtensions(&mypath) {
                messageMissingFiles();
            }
            // if good files have been selected - found only cvs and xlsx files
            else {
                break;
            }
        }
    }
    mypath
}

/// Check the extensions of files. Searches for other extensions than cvs or xlsx.
/// If found returns true
/// args: Vec<PathBUf> - vector of relative paths of files
pub fn checkFilesExtensions(path: &[PathBuf]) -> bool {

    let mybool = path.iter()
        .filter(|&my_path|my_path.extension().unwrap().to_str().unwrap() != "cvs" )
        .filter(|&my_path|my_path.extension().unwrap().to_str().unwrap() != "xlsx")
        .map(|my_path | my_path.is_file()
        ).collect::<Vec<bool>>();

    return mybool.len() > 0usize
}

/// Read directory to which the final result.pdf will be written
/// Return found directory
pub fn readDirectory() -> Option<PathBuf>{
    let directory = FileDialog::new().set_location(&env::current_dir().unwrap()).show_open_single_dir().unwrap();
    directory
}

pub fn messageDirectory(){
    MessageDialog::new().set_text("The result will be written to result.pdf file in given directory").show_alert().unwrap()

}

pub fn messageDone() {
    MessageDialog::new().set_text("Done. Please check result in PDF file").show_alert().unwrap()
}

pub fn messageMissingFiles() {
    MessageDialog::new().set_text("No such files. Please choose CVS and Xlsx files").show_alert().unwrap()
}

pub fn messageCancel() {
    MessageDialog::new().set_text("Operation cancelled").show_alert().unwrap()
}

pub fn messageEmptyFiles() {
    MessageDialog::new().set_text("Empty input files").show_alert().unwrap();
    exit(exitcode::OSFILE);
}


#[cfg(test)]
mod tests
{

    use super::*;
    use std::path::{PathBuf, Path};

    #[test]
    fn filterNonFiles(){
        let paths: Vec<PathBuf> = vec![
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy.pdf"),
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy2.pdf")
        ];
        assert_eq!(super::checkFilesExtensions(&paths),true)
    }

    #[test]
    fn filterCVSFile(){
        let paths: Vec<PathBuf> = vec![
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy.cvs"),
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy2.pdf")
        ];
        assert_eq!(super::checkFilesExtensions(&paths), true)
    }

    #[test]
    fn filterXlsxFile(){
        let paths: Vec<PathBuf> = vec![
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy.xlsx"),
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy2.pdf")
        ];
        assert_eq!(super::checkFilesExtensions(&paths), true)
    }


    #[test]
    fn filterCvsXlsxFiles(){
        let paths: Vec<PathBuf> = vec![
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy.cvs"),
            PathBuf::from(r"C:\Users\franc\Documents\nauka_rust\readAdobepython\nowy.xlsx")
        ];
        assert_eq!(super::checkFilesExtensions(&paths), false)
    }


}
