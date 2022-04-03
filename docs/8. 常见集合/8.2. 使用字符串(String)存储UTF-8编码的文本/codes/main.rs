
// TODO: # 8.2. 使用字符串(String)存储UTF-8编码的文本

fn main() {

  // 新建一个空的 Strings
  let mut s = String::new();
  println!("s: {}", &mut s); //  s:


  // 使用 to_string 方法从字符串字面值创建 String
  let data = "initial contents";
  let s = data.to_string();
  println!("s: {}", s); // s: initial contents

  // 该方未能也可直接用于字符串字面值
  let s = "initial contents".to_string();
  println!("s: {}", s); // s: initial contents

  // 也可以使用 String::from 函数来从字符串字面值创建 String。示例 8-13 中的代码等同于使用 to_string。
  /* 使用 String::from 函数从字符串字面值创建 String */
  let s = String::from("initial contents");
  println!("s: {}", s);  // s: initial contents

  // 记住字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据，如示例 8-14 所示。
  /* 在字符串中储存不同语言的问候语 所有这些都是有效的 String 值*/
  let hello = String::from("السلام عليكم");
  let hello = String::from("Dobrý den");
  let hello = String::from("Hello");
  let hello = String::from("שָׁלוֹם");
  let hello = String::from("नमस्ते");
  let hello = String::from("こんにちは");
  let hello = String::from("안녕하세요");
  let hello = String::from("你好");
  let hello = String::from("Olá");
  let hello = String::from("Здравствуйте");
  let hello = String::from("Hola");
  

  // 使用 push_str 和 push 附加字符串
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("s: {}", &mut s); // s: foobar


  // 将字符串 slice 的内容附加到 String 后使用它
  /*
   * 如果 push_str 方法获取了 s2 的所有权，就不能在最
   * 后一行打印出其值了。好在代码如我们期望那样工作！ 
   */
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2); // s2 is bar

  
  // 使用 push 将一个字符加入 String 值中
  /*
   * push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中。
   * 示例 8-17 展示了使用 push 方法将字母 "l" 加入 String 的代码。
   */
   let mut s = String::from("lo");
   s.push('l');
   println!("s: {}", &mut s); // s: lol


   // 使用 + 运算符或 format! 宏拼接字符串
   /**
    * 使用 + 运算符将两个 String 值合并到一个新的 String 值中
    * 
    * 通常你会希望将两个已知的字符串合并在一起。
    * 一种办法是像这样使用 + 运算符
    */
   let s1 = String::from("Hello,");
   let s2 = String::from("world!");

   let s3 = s1 + &s2;
   println!("s3: {}", s3); // s3: Hello,world!

   // 索引字符串
   let s1 = String::from("hello");
   let h = s1[0];

   let hello = "Здравствуйте";
   let answer = &hello[0];
   println!("answer: {}", answer);
}