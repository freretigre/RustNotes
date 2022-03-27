fn main() {
  // TODO: 6.3. if let 简洁控制流
  /*
   * 3u8 3 是 u8
   */
  let config_max = Some(3u8); 
  match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
  }
  // The maximum is configured to be 3

  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  }
  // The maximum is configured to be 3
  println!("3u8: {}", 8u8);  // 3u8: 8
 
  
  let config_max = Some(6u8); 
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  }
  
}