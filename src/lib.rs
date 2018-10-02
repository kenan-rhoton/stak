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

pub fn load_data() -> Option<Vec<String>> {
    let data = &data_file();

    match std::fs::read_to_string(data) {
        Ok(file) => Some(file
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(i, line)| format!("[{}] {}", i, line).to_string())
            .collect()),
        Err(_) => None,
    }
}

pub fn append_data(new_line: String) {
    let data = &data_file();

    let file = if std::path::Path::new(data).exists() {
        std::fs::OpenOptions::new().append(true).open(data)
    } else {
        std::fs::File::create(data)
    };

    match file {
        Ok(mut f) => {
            match f.write_fmt(format_args!("{}\n", new_line)) {
                Ok(_) => print!("Added: {}", new_line),
                Err(e) => eprintln!("Error while writing to file: {:?}", e),
            }
        },
        Err(e) => eprintln!("Error while opening the file: {:?}", e),
    };

}
