/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/

/**
 * 我们定义一个模块，是以 mod 关键字为起始，
 * 然后指定模块的名字（本例中叫做 front_of_house），
 * 并且用花括号包围模块的主体。在模块内，我们还可以定义
 * 其他的模块，就像本例中的 hosting 和 serving 模块。
 * 模块还可以保存一些定义的其他项，比如结构体、枚举、常量、
 * 特性、或者函数。
 */
/* mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
} */
// 一个包含了其他内置了函数的模块的 front_of_house 模块


mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    /**
     * 使用绝对路径和相对路径来调用 add_to_waitlist 函数
     */
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 创建公有的结构体和枚举
mod back_of_house {

    // 使用 pub 来设计公有的结构体和枚举
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


// 带有公有和私有字段的结构体
pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 改变注意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("Id'd like{} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；

    // 不允许查看或修改早餐附带的季节水果

    // meal.seasonal_fruit = String::from("blueberries");

}



mod back_of_house {
    // 设计公有枚举，使其所有成员公有
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}




