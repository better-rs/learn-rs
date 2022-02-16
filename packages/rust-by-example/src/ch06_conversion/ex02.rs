#[test]
fn ex02_try_from() {
    // TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    // TryFrom:
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(7), Err(()));

    // TryInto:
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    //
    let result: Result<EvenNumber, ()> = 7i32.try_into();
    assert_eq!(result, Err(()));
}
