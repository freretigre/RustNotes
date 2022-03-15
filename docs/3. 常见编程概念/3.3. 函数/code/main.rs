
fn main() {
  println!("Hello, World");
  // TODO: 3.3. 函数
  println!("==============函数===================");
  /*
   * Rust 代码中的函数和变量名使用下划线命名法（snake case，直译为蛇形命名法）规范风格。
   * 在下划线命名法中，所有字母都是小写并使用下划线分隔单词。
   */
  {
    another_function();
    
  }

  // TODO: 参数
  println!("==============参数===================");
  /*
   * 在函数签名中，必须声明每个参数的类型。这是一个 Rust 设计中经过
   * 慎重考虑的决定：要求在函数定义中提供类型标注，意味着编译器几乎从
   * 不需要你在代码的其他地方注明类型来指出你的意图。
   */
  {
    // 只有一个参数
    another_function_paraeter(8);

    // 一个函数有多个参数时，使用逗号分隔，像这样
    print_labeled_measurement(5, 'h');
   
  }

  // TODO: 语句和表达式
  println!("==============语句和表达式===================");
  {
    // let y = 6;

    // 这要是错误的写法
    // let x = (let y = 6);  

    // 正确的写法
    let x = {
      let y = 6;
      y + 1
    };
    println!("The value of y is: {}", x);
     // The value of y is: 7
  }


  // TODO: 带有返回值的函数
  println!("==============带有返回值的函数===================");
  {
    let x = five();
    println!("The value of x is: {}", x);
    // The value of x is: 5

    let x1 = plus_one(5);
    println!("The value of x is: {}", x1);
    // The value of x is: 6
  }

  

}

// 定义一个函数
fn another_function() {
  println!("Another function.");
  // Another function.
}

// 定义一个 传入参数的 方法
fn another_function_paraeter(x: i32) {
  println!("The value of x is: {}", x);
   // The value of x is: 8
}

// 一个函数有多个参数时
fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {} {}", value, unit_label);
  // The measurement is: 5 h
}

// 有返回值的函数的
fn five() -> i32 {
  5
}

// 不过函数体只有单单一个 5 也没有分号，因为这是一个表达式，正是我们想要返回的值。
fn plus_one(x: i32) -> i32 {
  x + 1
}