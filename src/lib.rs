use std::io::Write;
extern crate dirs;

const DATA_FILE: &str = ".stakdata";

fn data_file() -> String {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(DATA_FILE);
            path.to_string_lossy().to_string()
        },
        None => DATA_FILE.to_string(),
    }
}

pub fn load_data() -> Vec<String> {
    let data = &data_file();
    std::fs::read_to_string(data)
        .expect(&format!("File could not be read from \"{}\"!", data))
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect()
}

pub fn append_data(new_line: String) {
    let data = &data_file();
    let mut file = if std::path::Path::new(data).exists() {
        println!("APPENDING!");
        std::fs::OpenOptions::new().append(true).open(data)
    } else {
        println!("CREATING!");
        std::fs::File::create(data)
    }.expect("File could not be written to!");

    file.write_fmt(format_args!("{}\n", new_line.to_string()))
        .expect("Something went wrong!");
}
