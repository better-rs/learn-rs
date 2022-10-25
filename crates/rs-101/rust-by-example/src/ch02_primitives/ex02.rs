// 元组类型用法:
#[test]
fn ex02_tuples() {
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // 元组解引用
        let (integer, boolean) = pair;

        (boolean, integer) // 互换位置
    }

    ////////////////////////////////////////////////////////////////////////////////

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    //

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    println!("too long tuple: {}", too_long_tuple.0);

    ////////////////////////////////////////////////////////////////////////////////

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    ////////////////////////////////////////////////////////////////////////////////

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    ////////////////////////////////////////////////////////////////////////////////

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("tuple: ({:?}, {:?}, {:?}, {:?})", a, b, c, d);

    ////////////////////////////////////////////////////////////////////////////////

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix:{:?}", matrix);
}
