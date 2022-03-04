#[test]
fn ex01_if_else() {
    //
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    /*
        TODO X: 这种写法是很糟糕的设计
         虽然非常类似 Python 的三目表达式, 这种写法非常不清晰
         最夸张的是 IDE 既然会主动提示去掉 if()的括号
    */
    let big_n = if n < 10 && n > -10 {
        println!("{} has an interesting range", n);
        10 * n // 表达式返回
    } else {
        println!("{} is outside the interesting range", n);
        n / 2 // 表达式返回
    };

    println!("{} -> {}", n, big_n);
}
