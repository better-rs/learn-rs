use std::fmt::Debug;

#[test]
fn ex06_where() {
    /*
    TODO X:  等价写法:
        impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
        impl<A, D> MyTrait<A, D> for YourType
        where
            A: TraitB + TraitC,
            D: TraitE + TraitF,
        {
        }
    */

    trait PrintInOption {
        fn print_in_option(self);
    }

    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
