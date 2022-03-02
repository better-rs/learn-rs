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
fn ex04_01_explicit() {
    // TODO X: 显式标注生命周期
}
