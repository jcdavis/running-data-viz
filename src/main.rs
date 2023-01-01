use fitparser;

use std::env;
use std::fs;
use std::io;
use std::path::Path;


fn is_running(file: &Vec<fitparser::FitDataRecord>) -> bool {
    return match file.iter().find(|r| r.kind() == fitparser::profile::MesgNum::Sport)
        .and_then(|r| get_field_by_name(r, "name")){
        Some(r) => r.value() == &fitparser::Value::String("Run".to_string()),
        None => false,
    }
}

fn get_field_by_name<'a>(record: &'a fitparser::FitDataRecord, name: &str) -> Option<&'a fitparser:: FitDataField>{
    return record.fields().iter().find(|f| f.name() == name)
}

fn pretty_print_record(record: &fitparser::FitDataRecord) {
    println!("{}:", record.kind());
    for field in record.fields() {
        println!("  {}: {}", field.name(), field.value());
    }
}

fn get_fit_file(path: &Path) -> io::Result<Vec<fitparser::FitDataRecord>> {
    let mut file = fs::File::open(path)?;
    return fitparser::from_reader(&mut file).map_err(|e|
        io::Error::new(io::ErrorKind::Other, e)
    );
}

fn get_fit_files(dir: &str) -> io::Result<Vec<Vec<fitparser::FitDataRecord>>> {
    let files = fs::read_dir(dir)?;

    return files.into_iter().map(|file_result| {
        file_result.and_then(|file| get_fit_file(&file.path()))
    }).collect();
}

fn draw_chart(data: &Vec<usize>, outdir: &str) {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let activities = get_fit_files(&args[1]).unwrap();

    for activity in &activities {
        if is_running(activity) {
            let session = activity.iter().find(|r| r.kind() == fitparser::profile::MesgNum::Session).unwrap();
            let value_opt = get_field_by_name(session, "total_elapsed_time").map(|f| f.value());
            if let Some(fitparser::Value::Float64(d)) = value_opt {
                println!("{}", (d/60.0) as usize);
            }
        }
    }
}
