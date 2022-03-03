use std::fmt::Debug;

#[test]
fn ex04_lifetime() {
    /*
    TODO X:
        生命周期（lifetime）是这样一种概念，编译器（中的借用检查器）用它来保证所有的 借用都是有效的。
        确切地说，一个变量的生命周期在它创建的时候开始，在它销毁的时候 结束。
        虽然生命周期和作用域经常被一起提到，但它们并不相同。
    */

    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

    println!("i: {}", i);
}

#[test]
fn ex04_01_lifetime_explicit() {
    // TODO X: 显式标注生命周期

    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {}, y is {}", x, y);
    }

    fn failed_borrow<'a>() {
        let _x = 12;

        // error[E0597]:
        // let y: &'a i32 = &_x;
    }

    ////////////////////////////////////////////////////////////////////////////////

    let (four, nine) = (4, 9);

    // 引用传参
    print_refs(&four, &nine);

    failed_borrow();
}

#[test]
fn ex04_02_lifetime_fn() {
    ///
    /// todo x:
    ///     - 传参: 只读引用
    fn print_one<'a>(x: &'a i32) {
        println!("print_one: x = {}", x);
    }

    ///
    /// TODO X:
    ///     - 传数: 可变引用
    fn add_one<'a>(x: &'a mut i32) {
        // 类似 C 指针, 取值
        *x += 1;
    }

    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("print_multi: x = {}, y = {}", x, y);
    }

    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    // 值
    let mut t = 3;

    // todo x: 改变值: 传入的是可变引用
    add_one(&mut t);
    print_one(&t);
}

#[test]
fn ex04_03_lifetime_methods() {
    ///
    /// todo x: 单个属性, int32
    ///    - 方法一般是不需要标明生命周期的，因为 self 的生命周期会赋给所有的输出 生命周期参数
    ///
    struct Owner(i32);

    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }

        fn print<'a>(&'a self) {
            println!("print: {}", self.0);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

#[test]
#[allow(dead_code)]
fn ex04_04_lifetime_struct() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32); // 内嵌: 引用类型

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32, // 引用类型
        y: &'a i32,
    }

    // 枚举类型
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32), // 引用类型
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };

    // todo x: 枚举
    let reference = Either::Ref(&x);
    let number = Either::Num(x);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is not borrowed in {:?}", number);
}

#[test]
#[allow(dead_code)]
fn ex04_05_lifetime_trait() {
    #[derive(Debug)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    /// TODO X: trait 标注 'a
    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            // 默认值
            Self { x: &10 }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let b: Borrowed = Default::default();
    let c = Borrowed::default(); // 换一种写法

    println!("b is {:?}", b);
    println!("c is {:?}", c);
}

#[test]
fn ex04_06_lifetime_bounds() {
    use std::fmt::Debug;

    /// todo x: 泛型约束
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("`print`: t = {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("`print_ref`: t = {:?}", t);
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
