extern crate stak;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        match stak::load_data() {
            Some(data) => data.iter().for_each(|x| println!("{}", x)),
            None => eprintln!("No data found!"),
        }
    } else {
        let action: String = args[1..].join(" ");
        stak::append_data(action);
    }
}
