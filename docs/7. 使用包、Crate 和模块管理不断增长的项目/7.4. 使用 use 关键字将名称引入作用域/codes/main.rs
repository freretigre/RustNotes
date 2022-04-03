use std::collections::HashHap;
use std::fmt;
use std::io;
use std::fmt::Result;
use std::io::Result as IoResult;
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// 嵌套路径来消除大量的 use 行
/**
 * 相反，我们可以使用嵌套路径将相同的项在一行中引入作用域。这么做需要指定路径的相同部分，接着是两个冒号，接着是大括号中的各自不同的路径部分，如示例 7-18 所示。
 */
use std::{cmp::Ordering, io} // 指定嵌套的路径在一行中将多个带有相同前缀的项引入作用域
use std::{self, Write}; // 两个路径的相同部分是 std::io，这正是第一个路径。为了在一行 use 语句中引入这两个路径，可以在嵌套路径中使用 self



fn main() {
 
  let mut map = HashHap::new();
  map.insert(1, 2);


}

fn function1() -> fmt::Result {

}
fn function2() -> io::Result {

}

fn function3() -> Result {

}
fn function4() -> Result {

}
