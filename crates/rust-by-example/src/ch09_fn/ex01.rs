#[test]
fn ex01_fn() {
    //

    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }

    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    fizzbuzz_to(100);
}

#[test]
fn ex01_methods() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // 默认值:
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // 构造方法:
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            // TODO X: 解包: 得出 (x1, y1) 值, 类似 Python 元组解包
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            println!("unpack: ({}, {}), ({}, {})", x1, y1, x2, y2);

            // return:
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            // TODO X: 解包: 得出 (x1, y1) 值, 类似 Python 元组解包
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            println!("unpack: ({}, {}), ({}, {})", x1, y1, x2, y2);

            // return:
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    // TODO X: `Pair` 拥有资源：两个堆分配的整型
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        fn destroy(self) {
            // TODO X: 解包: 类似 Python 元组解包
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);
            // TODO X: `first` 和 `second` 离开作用域后释放
        }
    }
    ////////////////////////////////////////////////////////////////////////////////

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // pair.destroy(); // 不能再调用
}
