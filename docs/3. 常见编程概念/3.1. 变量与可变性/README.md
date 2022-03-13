# 变量和可变性
- 正如第二章中“使用变量储存值” 部分提到的那样，变量默认是不可改变的（immutable）。这是 Rust 提供给你的众多优势之一，让你得以充分利用 Rust 提供的安全性和简单并发性来编写代码。不过，你仍然可以使用可变变量。让我们探讨一下 Rust 为何及如何鼓励你利用不可变性，以及何时你会选择不使用不可变

- 当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值。为了对此进行说明，使用 cargo new variables 命令在 projects 目录生成一个叫做 variables 的新项目。

- 接着，在新建的 variables 目录，打开 src/main.rs 并将代码替换为如下代码，这些代码还不能编译，我们会首次检查到不可变错误（immutability error）。

- 文件名: src/main.rs
```rust
fn main() {
  println!("变量和可变性");
  
  let x = 5;
  log(x); // 打印的结果: 5

  x = 5;
  log(x);
}

fn log(msg: u64) {
  println!("打印的结果: {}", msg);
}
```
- 保存并使用 cargo run 运行程序。应该会看到一条错误信息，如下输出所示：
```shell
➜  rustkit git:(master) ✗ cargo run
   Compiling rustkit v0.1.0 (/Users/beiyong/FileRelated/other/rust/rustkit)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:7:3
  |
4 |   let x = 5;
  |       -
  |       |
  |       first assignment to `x`
  |       help: consider making this binding mutable: `mut x`
...
7 |   x = 5;
  |   ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `rustkit` due to previous error
```
- 这个例子展示了编译器如何帮助你找出程序中的错误。虽然编译错误令人沮丧，但那只是表示程序不能安全的完成你想让它完成的工作；并 不能 说明你不是一个好程序员！经验丰富的 Rustacean 们一样会遇到编译错误。


- 错误信息指出错误的原因是 不能对不可变变量 x 二次赋值（cannot assign twice to immutable variable `x` ），因为你尝试对不可变变量 x 赋第二个值。

- 在尝试改变预设为不可变的值时，产生编译时错误是很重要的，因为这种情况可能导致 bug。如果一部分代码假设一个值永远也不会改变，而另一部分代码改变了这个值，第一部分代码就有可能以不可预料的方式运行。不得不承认这种 bug 的起因难以跟踪，尤其是第二部分代码只是 有时 会改变值。

- Rust 编译器保证，如果声明一个值不会变，它就真的不会变，所以你不必自己跟踪它。这意味着你的代码更易于推导。

- 不过可变性也是非常有用的，可以用来更方便地编写代码。变量只是默认不可变；正如在第二章所做的那样，你可以在变量名之前加 mut 来使其可变。mut 也向读者表明了其他代码将会改变这个变量值的意图。

- 例如，让我们将 src/main.rs 修改为如下代码：

- 文件名: src/main.rs
```rust
fn main() {
  println!("变量和可变性");
  
  let mut x = 5;
  log(x); // 打印的结果: 5

  x = 6;
  log(x);
}

fn log(msg: u64) {
  println!("打印的结果: {}", msg);
}
```
- 现在运行这个程序，出现如下内容：
```shell
➜  rustkit git:(master) ✗ cargo run
   Compiling rustkit v0.1.0 (/Users/beiyong/FileRelated/other/rust/rustkit)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rustkit`
变量和可变性
打印的结果: 5
打印的结果: 6
```

- 通过 mut，允许把绑定到 x 的值从 5 改成 6。除了防止出现 bug 外，还有很多地方需要权衡取舍。例如，使用大型数据结构时，适当地使用可变变量，可能比复制和返回新分配的实例更快。对于较小的数据结构，总是创建新实例，采用更偏向函数式的编程风格，可能会使代码更易理解，为可读性而牺牲性能或许是值得的。
```rust
println!("变量和可变性");
  
  // TODO：变量和可变性
  /*
   * 在变量前面加 mut 是可变的
   * 如：let mut x = 5;
   * 
   * 在变量名前面不加 mut 是不可变的 
   * 如： let money = 169;    // 金额
   *
   */

  {
    // 在变量前面加 mut 是可变的
    let mut x = 5;
    log(x);             // 打印的结果: 5

    x = 6;
    log(x);             // 打印的结果: 6

    // 在变量名前面不加 mut 是不可变的 
    let money = 169;    // 金额
    let total = 896;    // 总计
    log(money);         // 打印的结果: 169
    log(total);         // 打印的结果: 896
  }
```


###  常量
- 类似于不可变变量，常量(constants) 是绑定到一个名称的不允许改变的值，不过常量与变量还是有一些区别。
- 首先，不允许对常量使用 mut。常量不光默认不能变，它总是不能变。
- 声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型。在下一部分，“数据类型” 中会介绍类型和类型注解，现在无需关心这些细节，记住总是标注类型即可。
- 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
- 最后一个区别是，`常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值`。
- 下面是一个声明常量的例子：
```rust
const EE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
- 在声明它的作用域之中，常量在整个程序生命周期中都有效，此属性使得常量可以作为多处代码使用的全局范围的值，例如一个游戏中所有玩家可以获取的最高分或者光速。
- 将遍布于应用程序中的硬编码值声明为常量，能帮助后来的代码维护人员了解值的意图。如果将来需要修改硬编码值，也只需修改汇聚于一处的硬编码值。
  
### 隐藏（Shadowing）
- 正如在第二章猜猜看游戏中所讲，我们可以定义一个与之前变量同名的新变量。Rustacean 们称之为第一个变量被第二个 隐藏 了，这意味着程序使用这个变量时会看到第二个值。可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏，如下所示：
- 文件名: src/main.rs
```rust
// TODO: 隐藏（Shadowing）
  println!("=====隐藏（Shadowing）=================");
  {
    let x = 5;
    let x = x + 1;
    log(x); // 印的结果: 6

    {
      let x = x * 2;
      log(x); // 印的结果: 12
    }
    log(x);   // 打印的结果: 6
  }
```
- 这个程序首先将 x 绑定到值 5 上。接着通过 let x = 隐藏 x，获取初始值并加 1，这样 x 的值就变成 6 了。然后，在内部作用域内，第三个 let 语句也隐藏了 x，将之前的值乘以 2，x 得到的值是 12。当该作用域结束时，内部 shadowing 的作用域也结束了，x 又返回到 6。运行这个程序，它会有如下输出：
```shell
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```
- 隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
- mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。例如，假设程序请求用户输入空格字符来说明希望在文本之间显示多少个空格，接下来我们想将输入存储成数字（多少个空格）：

```rust
    let spaces = "   ";
    let spaces = spaces.len();
```
- 第一个 spaces 变量是字符串类型，第二个 spaces 变量是数字类型。隐藏使我们不必使用不同的名字，如 spaces_str 和 spaces_num；相反，我们可以复用 spaces 这个更简单的名字。然而，如果尝试使用 mut，将会得到一个编译时错误，如下所示：
```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```
- 这个错误说明，我们不能改变变量的类型：
```shell
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```
- 现在我们已经了解了变量如何工作，让我们看看变量可以拥有的更多数据类型。


### 案例：main.rs 
```rust

fn main() {
  println!("变量和可变性");
  
  // TODO：变量和可变性
  println!("=====变量和可变性=================");
  /*
   * 在变量前面加 mut 是可变的
   * 如：let mut x = 5;
   * 
   * 在变量名前面不加 mut 是不可变的 
   * 如： let money = 169;    // 金额
   *
   */

  {
    // 在变量前面加 mut 是可变的
    let mut x = 5;
    log(x);             // 打印的结果: 5

    x = 6;
    log(x);             // 打印的结果: 6

    // 在变量名前面不加 mut 是不可变的 
    let money = 169;    // 金额
    let total = 896;    // 总计
    log(money);         // 打印的结果: 169
    log(total);         // 打印的结果: 896
  }

  // TODO: 常量
  println!("=====常量=================");
  {
    const EE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    log(EE_HOURS_IN_SECONDS); // 打印的结果: 10800
  }


  // TODO: 隐藏（Shadowing）
  println!("=====隐藏（Shadowing）=================");
  {
    let x = 5;
    let x = x + 1;
    log(x); // 印的结果: 6

    {
      let x = x * 2;
      log(x); // 印的结果: 12
    }
    log(x);   // 打印的结果: 6
  }

  {
    /* 
     * 第一个 spaces 变量是字符串类型，第二个 spaces 变量是数字类型。
     * 隐藏使我们不必使用不同的名字，如 spaces_str 和 spaces_num；
     * 相反，我们可以复用 spaces 这个更简单的名字。然而，如果尝试使用
     *  mut，将会得到一个编译时错误，如下所示：
     * 
     */
    let spaces = "      ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces); // spaces: 6

    /*
     *  第一个 spaces 变量是字符串类型，第二个 spaces 变量是数字类型。
     *  隐藏使我们不必使用不同的名字，如 spaces_str 和 spaces_num；相反，
     *  我们可以复用 spaces 这个更简单的名字。然而，如果尝试使用 mut，将会
     *  得到一个编译时错误，如下所示：
     */
    /* 
    let mut spaces = "   ";
    spaces = spaces.len();
    */
  }


  
}

fn log(msg: u32) {
  println!("打印的结果: {}", msg);
}

```