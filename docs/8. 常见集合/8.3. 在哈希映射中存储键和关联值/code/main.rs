use std::collections::HashMap;


// 哈希 map 储存键值对
fn main() {

  {
    // 新建一个哈希 map
    let mut scores = HashHap::new();

    // 新建一个哈希 map 并插入一些键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
  }


  // 构建哈希 map 的方法是在一个元组的 vector 上使用迭代器（iterator）和 collect 方法，其中每个元组包含一个键值对。
  /* 用队伍列表和分数列表创建哈希 map */
  {
    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_scoress = vec![10, 50];
    
    let mut scoress::HashHap<_, _> = teams.into_iter().zip(initial_scoress.into_iter()).collect();

  }


  // 哈希 map 和所有权
  /* 展示一旦键值对被插入后就为哈希 map 所拥有 */
  {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
  }


}

