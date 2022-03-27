#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  // TODO: 5.2. 一个使用结构体的示例程序

  /*
   * rectangles 的二进制程序，它获取以像素为单位
   * 的长方形的宽度和高度，并计算出长方形的面积。
   */
  {
    let width1 = 30;
    let height1 = 50;
 
    println!("The area of the rectangle is {} square pixels.", area1(width1, height1));
    // The area of the rectangle is 1500 square pixels.
  }
  

   // TODO: 使用元组重构
   {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area2(rect1));
    // The area of the rectangle is 1500 square pixels.
   }

   // TODO: 使用结构体重构：赋予更多意义
   /*
    定义 Rectangle 结构体
   */
   {
     let rect = Rectangle {
       width: 30,
       height: 30,
     };
     println!("The area of the rectangle is {} square pixels.", area3(&rect));
     // The area of the rectangle is 900 square pixels.

     // 增加属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例
     println!("rect is {:?}", rect);
     // rect is Rectangle { width: 30, height: 30 }

     /*
      * 好极了！这并不是最漂亮的输出，不过它显示这个
        实例的所有字段，毫无疑问这对调试有帮助。当我
        们有一个更大的结构体时，能有更易读一点的输出
        就好了，为此可以使用 {:#?} 替换 println! 
        字符串中的 {:?}。在这个例子中使用 {:#?} 风
        格将会输出：
      */
      println!("rect is {:#?}", rect);
      /*
      rect is Rectangle {
          width: 30,
          height: 30,
      }
      */

      /*
       * 另一种使用 Debug 格式打印数值的方法是
       * 使用 dbg! 宏。dbg! 宏接收一个表达式的所
       * 有权，打印出代码中调用 dbg! 宏时所在的文
       * 件和行号，以及该表达式的结果值，并返回该值
       * 的所有权。
       */

      let scall = 2;
      let rect1 = Rectangle {
        width: dbg!(30 * scall),
        height: 30,
      };

      dbg!(&rect1);

      /*
      [src/main.rs:72] 30 * scall = 60
      [src/main.rs:76] &rect1 = Rectangle {
          width: 60,
          height: 30,
      }
      */


   }
}

// 通过分别指定长方形的宽和高的变量来计算长方形面积
fn area1(width: u32, height: u32) -> u32 {
  width * height
}

// 使用元组来指定长方形的宽高
fn area2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

// 使用结构体重构：赋予更多意义
fn area3(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

