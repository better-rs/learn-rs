#[test]
fn ex01_trait() {
    /*
    TODO X:
        trait 是对未知类型 Self 定义的方法集。
        该类型也可以访问同一个 trait 中定义的 其他方法。
    */

    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        fn new(name: &'static str) -> Self;

        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked...", self.name);
            } else {
                println!("{} gets a haircut!", self.name);
                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Sheep {
            Sheep {
                naked: false,
                name: name,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name(), self.noise());
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

#[test]
fn ex01_trait_derive() {
    /*
    TODO X:
        通过 #[derive] 属性，编译器能够提供某些 trait 的基本实现。
        如果 需要更复杂的行为，这些 trait 也可以手动实现。
            比较 trait: Eq, PartialEq, Ord, PartialOrd
            Clone, 用来从 &T 创建副本 T。
            Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
            Hash，从 &T 计算哈希值（hash）。
            Default, 创建数据类型的一个空实例。
            Debug，使用 {:?} formatter 来格式化一个值。
    */

    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    struct Seconds(i32);

    ////////////////////////////////////////////////////////////////////////////////

    let _one_second = Seconds(1);
    // println!("One second is {:?}", _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}
