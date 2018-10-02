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

fn read_file_to_vec(filename: &str) -> std::io::Result<Vec<String>> {
    match std::fs::read_to_string(filename) {
        Ok(file) => Ok(file
                         .split("\n")
                         .filter(|line| !line.is_empty())
                         .map(|line| line.to_string())
                         .collect()),
        Err(e) => Err(e),
    }
}

fn write_vec_to_file(data: Vec<String>, filename: &str) -> std::io::Result<()>{
    match std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(filename) {
        Ok(mut file) => {
            let string_data: String = data.iter()
                .map(|line| format!("{}\n", line).to_string())
                .collect::<Vec<String>>()
                .concat();
            let byte_data: &[u8] = string_data.as_bytes();
            match file.write_all(byte_data) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn enumerate_data() -> std::io::Result<Vec<String>> {
    let data = &data_file();

    match read_file_to_vec(data) {
        Ok(file) => Ok(file.iter()
                           .enumerate()
                           .map(|(i, line)| format!("[{}] {}", i, line).to_string())
                           .collect()),
        Err(e) => Err(e),
    }
}

pub fn append_data(new_line: &String) -> std::io::Result<()>{
    let data = &data_file();

    let file = if std::path::Path::new(data).exists() {
        std::fs::OpenOptions::new().append(true).open(data)
    } else {
        std::fs::File::create(data)
    };

    match file {
        Ok(mut f) => {
            match f.write_fmt(format_args!("{}\n", new_line)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn remove_line(remove_num: i32) -> std::io::Result<String>{
    let data = &data_file();

    match read_file_to_vec(data) {
        Ok(mut file) => {
            if remove_num >= file.len() as i32 || remove_num < 0 {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Line does not exist"))
            } else {
                let removed = file.remove(remove_num as usize);
                match write_vec_to_file(file, data) {
                    Ok(_) => Ok(removed),
                    Err(e) => Err(e),
                }
            }
        },
        Err(e) => Err(e),
    }
}
