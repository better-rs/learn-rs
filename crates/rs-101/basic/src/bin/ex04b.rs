fn main() {
    let mut vec = vec!["Java".to_string(), "Rust".to_string(), "Python".to_string()];
    let mut vi = vec.iter_mut();

    while let Some(cs) = vi.next() {
        match cs.as_str() {
            "Rust" => {
                *cs = "Niubility".to_string();
                println!("{}", cs);
            },
            _ => println!("{}", cs),
        }
    }

    println!("{:?}", vec);
}
