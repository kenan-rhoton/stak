extern crate stak;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        match stak::enumerate_data() {
            Ok(data) => data.iter().for_each(|x| println!("{}", x)),
            Err(e) => eprintln!("No data found!\n{}", e),
        }
    } else {
        match args[1].as_ref() {
            "d" => {
                if args.len() > 2 {
                    match args[2].parse::<i32>() {
                        Ok(line_num) => match stak::remove_line(line_num) {
                            Ok(_) => (),
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Err(_) => eprintln!("You must provide a number!"),
                    }
                }
            }
            _ => {
                let data = args[1..].join(" ");
                match stak::append_data(&data) {
                    Ok(_) => println!("Added: {}", data),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
