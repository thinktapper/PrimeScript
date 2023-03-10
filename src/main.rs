fn main() {
    let file_name = std::env::args().nth(1).
        expect("Please provide a path to a file");

    let file = std::fs::read_to_string(file_name)
        .expect("Could not read file");

    file.lines().for_each(|line| println!("{}", line));
}