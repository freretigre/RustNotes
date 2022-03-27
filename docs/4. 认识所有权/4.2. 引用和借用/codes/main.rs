fn main() {
  // 引用与借用
  {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);
    println!("len: {}",len);
    // len: 5
  }
  
}

fn calculate_length(s: &String) -> usize{
  s.len()
}