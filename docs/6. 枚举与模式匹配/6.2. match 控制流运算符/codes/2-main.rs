// TODO: 6.2. match 控制流运算符

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

// 枚举
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

// 枚举
/**
 * Quarter 成员也存放了一个 UsState 值的 Coin 枚举
 */
enum Coina {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState), 
}





fn main() {

  // 调用 plus_one 方法
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("five: {:#?}", five);
  println!("six: {:#?}", six);
  println!("none: {:#?}", none);
}

// 以枚举成员作为模式的 match 表达式
fn _value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1, // 第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。这里的代码就仅仅是值 1
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn _value_in_centsa(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }, // 第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。这里的代码就仅仅是值 1
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

/*
fn value_in_centsb(coin: Coina) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
          println!("State quarter from {:?}!", state);
          25
      }
  }
}
*/

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}