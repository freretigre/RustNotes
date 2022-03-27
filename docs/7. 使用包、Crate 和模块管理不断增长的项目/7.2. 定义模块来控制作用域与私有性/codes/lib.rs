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
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
// 一个包含了其他内置了函数的模块的 front_of_house 模块
