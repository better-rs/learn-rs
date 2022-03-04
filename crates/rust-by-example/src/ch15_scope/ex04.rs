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

#[test]
fn ex04_07_lifetime_coercion() {
    /*
        TODO X:
            一个较长的生命周期可以强制转成一个较短的生命周期，使它在一个通常情况下不能工作 的作用域内也能正常工作。
            强制转换可由编译器隐式地推导并执行，也可以通过声明不同 的生命周期的形式实现。
    */

    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    /// todo x: 生命周期长度
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    ////////////////////////////////////////////////////////////////////////////////

    // 较长的生命周期
    let first = 2;
    {
        // 较短的生命周期
        let second = 3;

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}

#[test]
fn ex04_08_lifetime_static() {
    /*
    TODO X:
        'static 生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中 存在。
        'static 生命周期也可被强制转换成一个更短的生命周期
    */

    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    ////////////////////////////////////////////////////////////////////////////////

    {
        let static_string = "I'm in read-only memory";

        println!("static_string: {}", static_string);
    }

    ////////////////////////////////////////////////////////////////////////////////

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible", NUM);
}

#[test]
fn ex04_09_lifetime_elision() {
    /*
    TODO X:
        有些生命周期的模式太常用了，所以借用检查器将会隐式地添加它们以减少程序输入量 和增强可读性。
        这种隐式添加生命周期的过程称为省略（elision）。
    */

    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }

    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
