fn main() {
  
  {
    let guess: u32 = "43".parse().expect("Not a number!");
    println!("guess: {}", guess); // guess: 43
    // 这里如果不添加类型注解，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：
    /*

    expect() 这个方法就是注解

    $ cargo build
        Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
      error[E0282]: type annotations needed
      --> src/main.rs:2:9
        |
      2 |     let guess = "42".parse().expect("Not a number!");
        |         ^^^^^ consider giving `guess` a type

      For more information about this error, try `rustc --explain E0282`.
      error: could not compile `no_type_annotations` due to previous error

      你会看到其它数据类型的各种类型注解。
    */
    let _str: u8 = b'A';

  }


  // TODO: 浮点型
  /*
   * 浮点型 一般用于金额的计算
   * 
   * Rust 也有两个原生的 浮点数（floating-point numbers）类型，
   * 它们是带小数点的数字。Rust 的浮点数类型是 f32 和 f64，分别占
   * 32 位和 64 位。默认类型是 f64，因为在现代 CPU 中，它与 f32 
   * 速度几乎一样，不过精度更高。所有的浮点型都是有符号的。
   */
  {

    // 双精度
    let x1 = 2.0;  // 双精度，默认是 f64，可以不写
    // 也可以写成如下的方式，显得更加直观
    let x2: f64 = 126.0;
    println!("双精度x1: {}, 双精度x2: {}", x1, x2);
    // 双精度x1: 2, 双精度x2: 126

    // 单精度
    let y: f32 = 23.0; // 单精度必须写 f32
    println!("单精度y: {}", y);
    // 单精度y: 23
    
  }


  // TODO: 数值运算
  /* 
   * i 是否为正数，一般用于 在加+ 和 乘* 
   * u 是否为负数， 一般用于 在减- 和 除/
   * 
   */
  {
    // 加法
    let sum: i64 = 5 + 10;
    println!("加法 sum: {}", sum);
    
    // 减法
    let difference: f64 = 95.5 - 4.3;
    println!("减法 difference: {}", difference);

    // 乘法
    let produce: i64 = 4 * 30;
    println!("乘法 produce: {}", produce);

    // 除法
    let quotient: f64 = 56.7 / 32.2;
    let floored: i64 = 2 / 3; 
    println!("除法 quotient: {}", quotient);
    println!("除法 floored: {}", floored);
    
    // 取余
    let remainder: i64 = 43 % 5;
    println!("取余 remainder: {}", remainder);

    //============以上的写法完全可以写成如下的写法============
    // 加法
    let _sum = 5 + 10;

    // 减法
    let _difference = 95.5 - 4.3;

    // 乘法
    let _product = 4 * 30;

    // 除法
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // 结果为 0

    // 取余
    let _remainder = 43 % 5;

    /* 
     *  这些语句中的每个表达式使用了一个数学运算符并计算出了一个值，
     *  然后绑定给一个变量。附录 B 包含 Rust 提供的所有运算符的列表。
     *  https://kaisery.github.io/trpl-zh-cn/appendix-02-operators.html
     */

    // 把一个字符类型转换成一个数值的类型
    let num: f32 = "65.0".trim().parse().expect("你这个类型不对"); 
    println!("num: {}", num);
    
  }

  // TODO: 布尔型
  /*
   * 正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。
   * Rust 中的布尔类型使用 bool 表示。例如：
   * 
   * 
   */
  {
    let t = true;
    let f: bool = false; // 显式指定类型注解
    println!("布尔型 t bool: {}", t);
    // 布尔型 t bool: true
    println!("布尔型 f bool: {}", f);
    // 布尔型 f bool: false

    /*
     * 使用布尔值的主要场景是条件表达式，
     * 例如 if 表达式。在 “控制流”（“Control Flow”） 
     * 部分将介绍 if 表达式在 Rust 中如何工作。
     */
  }

  // TODO: 字符类型
  {
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    //=====以上也可以如下写法====
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    println!("字符类型 c: {}", c);
    println!("字符类型 z: {}", z);
    println!("字符类型 heart_eyed_cat: {}", heart_eyed_cat);
     /*
    字符类型 c: z
    字符类型 z: ℤ
    字符类型 heart_eyed_cat: 😻
    */

    let x = String::from("这是中文");
    println!("字符串类型 x: {}", x);
    /*
    字符串类型 x: 这是中文
    */
  }

  // TODO: 复合类型
  /*
   * 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
   * 
   * 元组类型
   *    元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
   *  
   *    我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值
   *    的类型也不必是相同的。这个例子中使用了可选的类型注解：
   */
  {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("复合类型：x: {}, y: {}, z: {}", x, y, z);
    // 复合类型：x: 500, y: 6.4, z: 1
  }

  {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("复合类型：x: {}, y: {}, z: {}", x, y, z);
    // 复合类型：x: 500, y: 6.4, z: 1
    /*
     * 程序首先创建了一个元组并绑定到 tup 变量上。接着使用了 let 和一个模式将 tup 
     * 分成了三个不同的变量，x、y 和 z。这叫做 解构（destructuring），因为它将一
     * 个元组拆成了三个部分。最后，程序打印出了 y 的值，也就是 6.4。
     * 
     */
  }

  {
    let x: (i32, f64, u8) = (800, 6.4, 2);
    // 我们也可以使用点号（.）后跟值的索引来直接访问它们。例如：
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("使用点号（.）后跟值的索引来直接访问它们: {}", five_hundred);
    println!("使用点号（.）后跟值的索引来直接访问它们: {}", six_point_four);
    println!("使用点号（.）后跟值的索引来直接访问它们: {}", one);
    /*
    使用点号（.）后跟值的索引来直接访问它们: 800
    使用点号（.）后跟值的索引来直接访问它们: 6.4
    使用点号（.）后跟值的索引来直接访问它们: 2
    */

    /*
     * 
     * 这个程序创建了一个元组，x，并接着使用索引为每个元素创建新变量。
     * 跟大多数编程语言一样，元组的第一个索引值是 0。
     * 
     * 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 () 。该类
     * 型被称为 单元类型（unit type），而该值被称为 单元值（unit value）。
     * 如果表达式不返回任何其他值，则会隐式返回单元值。
     */
     
  }



  // TODO: 数组类型
  /*
   * 另一个包含多个值的方式是 数组（array）。与元组不同，数组中的每个元素的类型必
   * 须相同。Rust 中的数组与一些其他语言中的数组不同，Rust中的数组长度是固定的。
   * 
   * 我们将数组的值写成在方括号内，用逗号分隔：
   */
  {

  }


}