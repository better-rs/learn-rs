fn main() {
    let mut vec = ["Java".to_string(), "Rust".to_string(), "Python".to_string()];
    for mut str in vec.iter_mut() {
        match str.as_str() {
            "Rust" => {
                *str = "Niubility".to_string();
                println!("{}", str);
            },
            _ => println!("{}", str),
        }
    }

    println!("{:?}", vec);
}
