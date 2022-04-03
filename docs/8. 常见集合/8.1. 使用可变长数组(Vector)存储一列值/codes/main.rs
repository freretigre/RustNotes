use std::fmt::Display;

fn main() {
  // 新建一个空的 vector 来储存 i32 类型的值
  // let v: Vec<i32> = Vec::new();

  /*
   * 通常，我们会用初始值来创建一个 Vec<T> 而 Rust
   *  会推断出储存值的类型，所以很少会需要这些类型注解。
   * 为了方便 Rust 提供了 vec! 宏，这个宏会根据我们提供的值来创建一个新的 vector。示例 8-2 新建一个拥有值 1、2 和 3 的 Vec<i32>。推断为 i32 是因为这是默认整型类型，第三章的 “数据类型” 讨论过：
   */
  let v1 = vec![1, 2, 3];
  println!("vec: {:?}", &v1[1]); // vec: 2

  // 更新 vector
  /*
   * 使用 push 方法向 vector 增加值
   */
  let mut v = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  println!("get push v value: {}", &v[1]); // get push v value: 6

  // 读取 vector 的元素
  let v = vec![1, 2, 3, 4, 5];
  
  let third: &i32 = &v[2];
  println!("third: {}", third); // third: 3


  // 使用索引语法或 get 方法来访问 vector 中的项
  match v.get(2) {
    Some(thirdval) => println!("The thirdval elementis {}", thirdval), // The thirdval elementis 3
    None => println!("There is no third element."),
  }

  // let v = vec![1, 2, 3, 5];
  // let does_not_exista = &v[100]; // 越界
  // let does_not_existb = v.get(100); // 越界 

  // println!("does_not_exista: {}", does_not_exista);
  // println!("does_not_existb: {}", does_not_existb);

  
  // let mut v = vec![1, 2, 3, 4, 5];
  // let first = &v[0];
  // println!("The first element is: {}", first);

  // 遍历 vector 中的元素
  /*
   * 如果想要依次访问 vector 中的每一个元素，我们可以遍历其所有的
   * 元素而无需通过索引一次一个的访问。示例 8-8 展示了如何使用 for 
   * 循环来获取 i32 值的 vector 中的每一个元素的不可变引用并将其打印：
   */
  let v = vec![100, 32, 54];
  for i in &v {
    println!("{}", i);
  }
  /*
  100
  32
  54
  */


  /*
   * 我们也可以遍历可变 vector 的每一个元素的可变引用以便能改变
   * 他们。示例 8-9 中的 for 循环会给每一个元素加 50：
   */
  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
    
    println!("i: {}", i);
  }
  /*
  i: 150
  i: 82
  i: 107
  */
  // 遍历 vector 中元素的可变引用

  // 使用枚举来储存多种类型
  enum SpeadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpeadsheetCell::Int(3),
    SpeadsheetCell::Float(10.12),
    SpeadsheetCell::Text(String::from("Blue")),
  ];
  
}