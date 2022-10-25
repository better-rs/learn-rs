#[test]
fn ex01_raii() {
    // TODO X: Box<T> 占有 堆（heap）中的内存
    fn create_box() {
        let _box1 = Box::new(3i32);
    }

    let _box2 = Box::new(5i32);

    // TODO X: 局部作用域: 生命周期
    {
        let _box3 = Box::new(4i32);
    }

    // range:
    for _ in 0u32..1_000 {
        create_box();
    }

    println!("{}", _box2);
    // println!("{}", _box3);
}

#[test]
fn ex01_01_drop() {
    #[derive(Debug)]
    struct ToDrop;

    // TODO X: 析构函数
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped!");
        }
    }

    let x = ToDrop;

    println!("made a ToDrop: {:?}", x);
}
