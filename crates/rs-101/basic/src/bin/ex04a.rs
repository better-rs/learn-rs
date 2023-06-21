fn main() {
    let mut vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter_mut() {
        let &mut cs = str;
        match cs {
            "Rust" => {
                *str = "Niubility";
                println!("{}", str);
            },
            _ => println!("{}", str),
        }
    }

    println!("{:?}", vec);
}
