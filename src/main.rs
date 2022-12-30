use fitparser;

use std::env;
use std::fs;
use std::io;


fn get_fit_files(dir: &str) -> io::Result<Vec<Vec<fitparser::FitDataRecord>>> {
    let files = fs::read_dir(dir)?;

    return files.into_iter().map(|file_result| {
        file_result.and_then(|file| fs::File::open(file.path())).and_then(|mut f| {
            match fitparser::from_reader(&mut f) {
                Ok(data) => {
                    println!("Read {} records", data.len());
                    Ok(data)
                },
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            }
        })
    }).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let data = get_fit_files(&args[1]).unwrap();

    println!("{}", data.len());
}
