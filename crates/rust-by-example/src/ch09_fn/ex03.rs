#[test]
fn ex03_hof() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("find the sum of all the squared odd numbers under 1000");
    let upper = 1000;
    let mut acc = 0;

    //  写法1:
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    // TODO X: 写法2: 函数式写法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);

    println!("functional style: {}", sum_of_squared_odd_numbers);
}
