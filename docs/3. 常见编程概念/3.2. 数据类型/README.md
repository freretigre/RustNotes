# 数据类型
- 在 Rust 中，每一个值都属于某一个 `数据类型（data type）`，这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。


- 记住，Rust 是 `静态类型（statically typed）`语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，比如第二章的 “比较猜测的数字和秘密数字” 使用 parse 将 String 转换为数字时，必须增加类型注解，像这样：
```rust
let guess: u32 = "32".parse().expect("Not a number!");
```
```rust
fn main() {
  
  {
    let guess: u32 = "43".parse().expect("Not a number!");
    println!("guess: {}", guess); // guess: 43
    // 这里如果不添加类型注解，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：
    /*
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
  }


}
```
- 这里如果不添加类型注解，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：
```shell
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error

# 你会看到其它数据类型的各种类型注解。
```

### 标量类型
- `标量（scalar）`类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。你可能在其他语言中见过它们。让我们深入了解它们在 Rust 中是如何工作的。
  
***`整型`***
- 整数 是一个没有小数部分的数字。我们在第二章使用过 u32 整数类型。该类型声明表明，它关联的值应该是一个占据 32 比特位的无符号整数（有符号整数类型以 i 开头而不是 u）。表格 3-1 展示了 Rust 内建的整数类型。我们可以使用其中的任一个来声明一个整数值的类型。
**表格 3-1: Rust 中的整型**
```rust
长度	       有符号	   无符号
8-bit	    i8	        u8
16-bit	    i16	        u16
32-bit	    i32	        u32
64-bit	    i64	        u64
128-bit	    i128	    u128
arch	    isize	    usize
```
- 每一个变体都可以是有符号或无符号的，并有一个明确的大小。`有符号` 和 `无符号` `代表数字能否为负值`，换句话说，`这个数字是否有可能是负数（有符号数`），`或者永远为正而不需要符号（无符号数）`。这有点像在纸上书写数字：当需要考虑符号的时候，数字以加号或减号作为前缀；然而，可以安全地假设为正数时，加号前缀通常省略。有符号数以补码形式（two’s complement representation） 存储。
- 可以看这遍文章：`https://en.wikipedia.org/wiki/Two%27s_complement`
```rust
位值	        无符号值	    补码

0000 0000	    0	     0
0000 0001	    1	     1
0000 0010	    2	     2
0111 1110	    126	     126
0111 1111	    127	     127
1000 0000	    128	     -128
1000 0001	    129	     -127
1000 0010	    130	     -126
1111 1110	    254	     -2
1111 1111	    255	     -1

fn main() {
    /**
     *  `有符号` 和 `无符号` `代表数字能否为负值`
     * 
     *   换句话说： 这个数字是否有可能是负数（有符号数）
     *            或者永远为正而不需要符号（无符号数）
     *      
     */
    let money: i32 = 168; // 金额有可能是负数，所以要用 i 开头的
    let price: u32 = 58;  // 单价一般是为正数，所以要用 u 开头的
}
```

- 每一个有符号的变体可以储存包含从 -(2n - 1) 到 2n - 1 - 1 在内的数字，这里 n 是变体使用的位数。所以 i8 可以储存从 -(27) 到 27 - 1 在内的数字，也就是从 -128 到 127。无符号的变体可以储存从 0 到 2n - 1 的数字，所以 u8 可以储存从 0 到 28 - 1 的数字，也就是从 0 到 255。

- 另外，isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

- 可以使用表格 3-2 中的任何一种形式编写数字字面值。请注意可以是多种数字类型的数字字面值允许使用类型后缀，例如 57u8 来指定类型，同时也允许使用 _ 做为分隔符以方便读数，例如1_000，它的值与你指定的 1000 相同。

***表格 3-2: Rust 中的整型字面值***
```rust
数字字面值	                        例子
Decimal (十进制)                     98_222
Hex (十六进制)	                    0xff
Octal (八进制)	                    0o77
Binary (二进制)	                    0b1111_0000
Byte (单字节字符)(仅限于u8)	        b'A'
```
- 那么该使用哪种类型的数字呢？如果拿不定主意，Rust 的默认类型通常是个不错的起点，数字类型默认是 i32。isize 或 usize 主要作为某些集合的索引。
****整型溢出****
> 比方说有一个 u8 ，它可以存放从零到 255 的值。那么当你将其修改为 256 时会发生什么呢？这被称为 “整型溢出”（“integer overflow” ），这会导致以下两种行为之一的发生。当在 debug 模式编译时，Rust 检查这类问题并使程序 panic，这个术语被 Rust 用来表明程序因错误而退出。第九章 “panic! 与不可恢复的错误” 部分会详细介绍 panic。

> 在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（two’s complement wrapping）的操作。简而言之，值 256 变成 0，值 257 变成 1，依此类推。依赖整型溢出被认为是一种错误，即便可能出现这种行为。如果你确实需要这种行为，标准库中有一个类型显式提供此功能，Wrapping。 为了显式地处理溢出的可能性，你可以使用标准库在原生数值类型上提供的以下方法:

    - 所有模式下都可以使用 wrapping_* 方法进行包装，如 wrapping_add
    - 如果 check_* 方法出现溢出，则返回 None值
    - 用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出
    - 用 saturating_* 方法在值的最小值或最大值处进行饱和处理


### 浮点型
- Rust 也有两个原生的 `浮点数（floating-point numbers`）类型，它们是带小数点的数字。Rust 的浮点数类型是 `f32` 和 `f64`，分别占 `32` 位和 `64` 位。默认类型是 `f64`，因为在现代 CPU 中，它与 `f32` 速度几乎一样，不过精度更高。所有的浮点型都是有符号的。

- 这是一个展示浮点数的实例：
- 文件名: src/main.rs
```rust
fn main() {
    let x = 2.0;        // 如果不写 x:f64 就是默认的是 f64 为双精度, 如果写上看起来更佳直观
    let y: f32 = 3.0;   // 如果写 y:f32 就是默认的是单精度

    // 一般金额方面都是双精度
    let money: f64 = 126.0;

}
```
```rust

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
```
***浮点数采用 IEEE-754 标准表示。f32 是单精度浮点数，f64 是双精度浮点数。***

### 数值运算
> Rust 中的所有数字类型都支持基本数学运算：`加法、减法、乘法、除法和取余`。整数除法会向下舍入到最接近的整数。下面的代码展示了如何在 let 语句中使用它们：

- 文件名: src/main.rs
```rust
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
```
- 这些语句中的每个表达式使用了一个数学运算符并计算出了一个值，然后绑定给一个变量。附录 B 包含 Rust 提供的所有运算符的列表。


### 布尔型
> 正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool 表示。例如：

- 文件名: src/main.rs
```rust
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
```

### 字符类型
- Rust的 char 类型是语言中最原生的字母类型。下面是一些声明 char 值的例子：
- 文件名: src/main.rs
```rust
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

```
> 注意，我们用单引号声明 char 字面量，而与之相反的是，使用双引号声明字符串字面量。Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。不过，“字符” 并不是一个 Unicode 中的概念，所以人直觉上的 “字符” 可能与 Rust 中的 char 并不符合。第八章的 “使用字符串存储 UTF-8 编码的文本” 中将详细讨论这个主题。


### 复合类型
- `复合类型（Compound types）`可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

- `元组类型`
  - 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。

  - 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。这个例子中使用了可选的类型注解：
- 文件名: src/main.rs
```rust
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

```
- 文件名: src/main.rs
```rust
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
```
- 这个程序创建了一个元组，x，并接着使用索引为每个元素创建新变量。跟大多数编程语言一样，元组的第一个索引值是 0。
- 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 () 。该类型被称为 `单元类型（unit type）`，而该值被称为 `单元值（unit value）`。如果表达式不返回任何其他值，则会隐式返回单元值。

### 数组类型
- 另一个包含多个值的方式是 `数组（array）`。与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，Rust中的数组长度是固定的。

- 我们将数组的值写成在方括号内，用逗号分隔：
- 文件名: src/main.rs

