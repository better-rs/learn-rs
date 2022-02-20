use std::fmt::{Debug, Display};

#[test]
#[allow(dead_code)]
fn ex04_bounds() {
    /*
    TODO X:
        在使用泛型时，类型参数常常必须使用 trait 作为约束（bound）来明确规定 类型应实现哪些功能。

    */

    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    struct S<T: Display>(T);

    // let s = S(vec![[1]]);
    // println!("s = {}", s);

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    struct Triangle {
        length: f64,
        height: f64,
    }

    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }
    ////////////////////////////////////////////////////////////////////////////////

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
}
