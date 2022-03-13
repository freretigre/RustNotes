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

  {
    let _name = "大豆（CBT）";
    let _money = 1705.32;



  }


  
}

fn log(msg: u32) {
  println!("打印的结果: {}", msg);
}

