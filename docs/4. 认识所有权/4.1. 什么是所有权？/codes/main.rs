fn main() {
  // 什么是所有权？

  // TODO: 所有权与函数
  {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
  }


  // TODO: 返回值与作用域
  {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1
    println!("s1: {}", s1);             // s1: Yours                                  

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
    println!("s3: {}", s3);             // s3: hello  

  }

  // 我们可以使用元组来返回多个值
  {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    // The length of 'hello' is 5.
  }

}// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
 // 所以不会有特殊操作
 // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
 // 所以什么也不会发生。s1 移出作用域并被丢弃

fn takes_ownership(some_string: String) { // some_string 进入作用域
  println!("{}", some_string);   // hello
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);    // 5
}// 这里，some_integer 移出作用域。不会有特殊操作
// 带有所有权和作用域注释的函数

// 返回值与作用域
fn gives_ownership() -> String { // gives_ownership 将返回值移动给
                                 // 调用它的函数
  let some_string = String::from("Yours"); // some_string 进入作用域
  
  some_string                              // 返回 some_string 并移出给调用的函数
  
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
  a_string  // 返回 a_string 并移出给调用的函数
}


// 我们可以使用元组来返回多个值
fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() 返回字符串的长度
  (s, length)
}
//  返回参数的所有权