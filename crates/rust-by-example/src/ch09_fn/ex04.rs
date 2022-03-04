#[test]
#[allow(dead_code)]
fn ex04_diverging() {
    fn foo() -> ! {
        panic!("This function never returns!");
    }

    // return
    fn some_fn() {
        ()
    }

    let a = some_fn();

    println!("this function returns and you can see this line: {:?}", a);

    ////////////////////////////////////////////////////////////////////////////////

    // let x = panic!("This function never returns!");
    // println!("You will never see this line!");

    ////////////////////////////////////////////////////////////////////////////////

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;

        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };

            // sum:
            acc += addition;
        }

        return acc;
    }

    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}
