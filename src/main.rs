extern crate stak;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let data: Vec<String> = stak::load_data();
        data.iter().for_each(|x| println!("{}", x));
    } else {
        let action: String = args[1..].join(" ");
        println!("Adding: {:?}", action);
        stak::append_data(action);
    }
}
