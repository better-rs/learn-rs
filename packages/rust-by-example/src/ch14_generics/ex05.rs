use std::fmt::Debug;
use std::fmt::Display;

#[test]
fn ex05_multi_bounds() {
    //

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: {:?}", t);
        println!("Display: {}", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t = {:?}, u = {:?} ", t, u);
    }

    ////////////////////////////////////////////////////////////////////////////////

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array);

    compare_types(&array, &vec);
}
