#[test]
fn ex03_while() {
	//

	let mut n = 1;

	while n < 101 {
		// check
		if n % 15 == 0 {
			println!("fizzbuzz");
		} else if n % 3 == 0 {
			println!("fizz");
		} else if n % 5 == 0 {
			println!("buzz");
		} else {
			println!("{}", n);
		}

		// count
		n += 1;
	}
}
