use std::io::Write;

const DATA_FILE: &str = "data";

pub fn load_data() -> Vec<String> {
        std::fs::read_to_string(DATA_FILE)
            .expect("File could not be read from!")
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect()
}

pub fn append_data(new_line: String) {
    let mut file = if std::path::Path::new(DATA_FILE).exists() {
        println!("APPENDING!");
        std::fs::OpenOptions::new().append(true).open(DATA_FILE)
    } else {
        println!("CREATING!");
        std::fs::File::create(DATA_FILE)
    }.expect("File could not be written to!");

    file.write_fmt(format_args!("\n{}", new_line.to_string()))
        .expect("Something went wrong!");
}
