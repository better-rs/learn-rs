#[test]
fn ex03_gen_trait() {
    struct Empty;
    struct Null;

    // 接口定义:
    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }

    // 实现:
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {
            println!("Dropping...");
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty;
    // null;
}
