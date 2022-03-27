


 // User 结构体定义
 struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  // 5.1. 定义并实例化结构体
  {

   

    // 创建 User 结构体的实例
    let user1 = User {
      active: true,
      username: String::from("someusername123"),
      email: String::from("someone@example.com"),
      sign_in_count: 1,
    };

    // 改变 User 实例 email 字段的值
    // user1.email = String::from("anotheremail@example.com");


    // 使用结构体更新语法从其他实例创建实
    /*
     * 使用 user1 中的一个值创建一个新的 User 实例
     */
   /*  let user2 = User {
      active: user1.active,
      username: user1.username,
      email: String::from("another@example.com"),
      sign_in_count: user1.sign_in_count,
    }; */

    // 使用结构体更新语法为一个 User 实例设置一个新的 email 值，不过其余值来自 user1 变量中实例的字段
    let user2 = User {
      email: String::from("another@example.com"),
      ..user1
    };

  }

  // TODO: 使用没有命名字段的元组结构体来创建不同的类型
  {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
  }

  // TODO: 没有任何字段的类单元结构体
  {
    struct AlwaysEqual;
    let subject = AlwaysEqual;
  }

  // TODO: 结构体数据的所有权
  {
    struct Users {
      active: bool,
      username: &str,
      email: &str,
      sign_in_count: u64,
    }

    let user1 = Users {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
    // 编译器会抱怨它需要生命周期标识符：
    /*
      $ cargo run
        Compiling structs v0.1.0 (file:///projects/structs)
      error[E0106]: missing lifetime specifier
      --> src/main.rs:3:15
        |
      3 |     username: &str,
        |               ^ expected named lifetime parameter
        |
      help: consider introducing a named lifetime parameter
        |
      1 ~ struct User<'a> {
      2 |     active: bool,
      3 ~     username: &'a str,
        |

      error[E0106]: missing lifetime specifier
      --> src/main.rs:4:12
        |
      4 |     email: &str,
        |            ^ expected named lifetime parameter
        |
      help: consider introducing a named lifetime parameter
        |
      1 ~ struct User<'a> {
      2 |     active: bool,
      3 |     username: &str,
      4 ~     email: &'a str,
        |

      For more information about this error, try `rustc --explain E0106`.
      error: could not compile `structs` due to 2 previous errors

      第十章会讲到如何修复这个问题以便在结构体中存储引用，不过现在，我们会使用像 String 这类拥有所有权的类型来替代 &str 这样的引用以修正这个错误。
    */
  }


}

// build_user 函数获取 email 和用户名并返回 User 实例
/* fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username: username,
    email: email,
    sign_in_count: 1,
  }
} */

// build_user 函数使用了字段初始化简写语法，因为 email 和 username 参数与结构体字段同名
/**
 * 这里我们创建了一个新的 User 结构体实例，
 * 它有一个叫做 email 的字段。我们想要将 email
 * 字段的值设置为 build_user 函数 email 参数的值。
 * 因为 email 字段与 email 参数有着相同的名称，则
 * 只需编写 email 而不是 email: email。
 */
fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}