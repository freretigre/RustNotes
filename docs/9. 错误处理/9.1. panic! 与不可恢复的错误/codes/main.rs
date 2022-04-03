// panic! 与不可恢复的错误
fn main() {
  panic!("crash and burn");

  let v = vec![1, 2, 3];
  v[99];
}