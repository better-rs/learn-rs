#[test]
fn ex04_for() {
	// v1:
	for n in 1..101 {
		if n % 15 == 0 {
			println!("FizzBuzz");
		} else if n % 3 == 0 {
			println!("Fizz");
		} else if n % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", n);
		}
	}

	// v2:
	for x in 1..=100 {
		if x % 15 == 0 {
			println!("FizzBuzz");
		} else if x % 3 == 0 {
			println!("Fizz");
		} else if x % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", x);
		}
	}
}

#[test]
fn ex04_for_iter() {
	let names = vec!["Bob", "Frank", "Ferris"];

	/*
	TODO X:
		iter - 在每次迭代中借用集合中的一个元素。
		这样集合本身不会被改变，循环之后仍可以使用。
	*/
	for name in names.iter() {
		match name {
			&"Ferris" => println!("There is a rustacean among us!"),

			// default
			_ => println!("Hello {}", name),
		}
	}

	////////////////////////////////////////////////////////////////////////////////

	/*
	TODO X:
		into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。
		一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
	*/
	for name in names.into_iter() {
		match name {
			"Ferris" => println!("There is a rustacean among us!"),

			// default
			_ => println!("Hello {}", name),
		}
	}

	////////////////////////////////////////////////////////////////////////////////

	// 覆盖:
	let mut names = vec!["Bob", "Frank", "Ferris"];
	println!("before: names: {:?}", names);

	// TODO X: iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
	for name in names.iter_mut() {
		// TODO X: 修改值(赋值方式)
		*name = match name {
			&mut "Ferris" => "There is a rustacean among us!",

			// default
			_ => "Hello",
		}
	}

	println!("after: names: {:?}", names);
}
