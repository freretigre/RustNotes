/**
 * 方法语法
 * 
 * 方法（method）与函数类似：它们使用 fn 关键字和名称声明，
 * 可以拥有参数和返回值，同时包含在某处调用该方法时会执行的
 * 代码。不过方法与函数是不同的，因为它们在结构体的上下文中
 * 被定义（或者是枚举或 trait 对象的上下文，将分别在第六章
 * 和第十七章讲解），并且它们第一个参数总是 self，它代表调
 * 用该方法的结构体实例。
 */

// ==========1================
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// 实现的获取一个 Rectangle 实例作为参数的 area 函数
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

// =========2=================
#[derive(Debug)]
struct Rectanglea {
  width: u32,
  // height: u32,
}
impl Rectanglea {
  fn width(&self) -> bool {
    self.width > 0
  }
}


// =========3=================
impl Rectangle {
  fn areab(&self) -> u32 {
      self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }
}

fn main() {
  // ==========1===============
  // TODO: 定义方法
  /*
   * 让我们把前面实现的获取一个 Rectangle 实例作为参数的 
   * area 函数，改写成一个定义于 Rectangle 结构体上的 
   * area 方法，如示例 5-13 所示：
   */
  {
    let rect = Rectangle {
      width: 30,
      height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());
    // The area of the rectangle is 1500 square pixels.
    // 在 Rectangle 结构体上定义 area 方法
  }

  // ===========2===============
  {
    let rect = Rectanglea {
      width: 30,
      // height: 50,
    };

    if rect.width() {
      println!("The rectangle has a nonzero width; it is {}", rect.width());
      // The rectangle has a nonzero width; it is true
    }
  }


  // TODO: 带有更多参数的方法
  {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /*
    使用还未实现的 can_hold 方法
    同时我们希望看到如下输出，因为 rect2 的两个维度都小于 rect1，而 rect3 比 rect1 要宽：
    Can rect1 hold rect2? true
    Can rect1 hold rect3? false
    */

  }

}

