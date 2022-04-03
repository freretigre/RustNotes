# trait：定义共享的行为
> trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

- 注意：trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。

### 定义 trait
> 一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

> 例如，这里有多个存放了不同类型和属性文本的结构体：结构体 NewsArticle 用于存放发生于世界各地的新闻故事，而结构体 Tweet 最多只能存放 280 个字符的内容，以及像是否转推或是否是对推友的回复这样的元数据。

> 我们想要创建一个名为 aggregator 的多媒体聚合库用来显示可能储存在 NewsArticle 或 Tweet 实例中的数据的总结。每一个结构体都需要的行为是他们是能够被总结的，这样的话就可以调用实例的 summarize 方法来请求总结。示例 10-12 中展示了一个表现这个概念的公有 Summary trait 的定义：

- 文件名: src/lib.rs
```rust
pub trait Sumar {
    fn summarize(&self) -> String;
}
```
> 示例 10-12：Summary trait 定义，它包含由 summarize 方法提供的行为

> 这里使用 trait 关键字来声明一个 trait，后面是 trait 的名字，在这个例子中是 Summary。我们也声明 trait 为 pub 以便依赖这个 crate 的 crate 也可以使用这个 trait，正如我们见过的一些示例一样。在大括号中声明描述实现这个 trait 的类型所需要的行为的方法签名，在这个例子中是 fn summarize(&self) -> String。

> 在方法签名后跟分号，而不是在大括号中提供其实现。接着每一个实现这个 trait 的类型都需要提供其自定义行为的方法体，编译器也会确保任何实现 Summary trait 的类型都拥有与这个签名的定义完全一致的 summarize 方法。

> trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。

### 为类型实现 trait
> 现在我们定义了 Summary trait 的签名，接着就可以在多媒体聚合库中实现这个类型了。示例 10-13 中展示了 NewsArticle 结构体上 Summary trait 的一个实现，它使用标题、作者和创建的位置作为 summarize 的返回值。对于 Tweet 结构体，我们选择将 summarize 定义为用户名后跟推文的全部文本作为返回值，并假设推文内容已经被限制为 280 字符以内。

- 文件名: src/lib.rs
```rust
// 创建一个结构体
pub struct NeewArticle{
    pub handline: String,
    pub location: String,
    pub author: Stringh,
    pub content: String,
}

// 展示了 NewsArticle 结构体上 Summary trait 的一个实现, 它使用标题、作者和创建的位置作为 summarize 的返回值
impl Summary for NewsArticle {
    fn sumarize(&self) -> String {
        format!("{}, by {} ({})", self.handline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self,content)
    }
}
```
> 在 NewsArticle 和 Tweet 类型上实现 Summary trait

> 在类型上实现 trait 类似于实现与 trait 无关的方法。区别在于 impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称。在 impl 块中，使用 trait 定义中的方法签名，不过不再后跟分号，而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的行为。

> 现在库在 NewsArticle 和 Tweet 上实现了Summary trait，crate 的用户可以像调用常规方法一样调用 NewsArticle 和 Tweet 实例的 trait 方法了。唯一的区别是 trait 必须和类型一起引入作用域以便使用额外的 trait 方法。这是一个二进制 crate 如何利用 aggregator 库 crate 的例子：

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
>  这会打印出 1 new tweet: horse_ebooks: of course, as you probably already know, people。

> 其他依赖 aggregator crate 的 crate 也可以将 Summary 引入作用域以便为其自己的类型实现该 trait。实现 trait 时需要注意的一个限制是，只有当至少一个 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。例如，可以为 aggregator crate 的自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 aggregator crate 本地的作用域中。类似地，也可以在 aggregator crate 中为 Vec<T> 实现 Summary，这是因为 Summary trait 位于 aggregator crate 本地作用域中。

> 但是不能为外部类型实现外部 trait。例如，不能在 aggregator crate 中为 Vec<T> 实现 Display trait。这是因为 Display 和 Vec<T> 都定义于标准库中，它们并不位于 aggregator crate 本地作用域中。这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。

### 默认实现
> 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。

> 示例 10-14 中展示了如何为 Summary trait 的 summarize 方法指定一个默认的字符串值，而不是像示例 10-12 中那样只是定义方法签名：

- 文件名: src/lib.rs
```rust
pub trait summary {
    fn summarize(&self) -> String {
        String::from("Read mor...")
    }
}
```
> 示例 10-14：Summary trait 的定义，带有一个 summarize 方法的默认实现

> 如果想要对 NewsArticle 实例使用这个默认实现，而不是定义一个自己的实现，则可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块。

> 虽然我们不再直接为 NewsArticle 定义 summarize 方法了，但是我们提供了一个默认实现并且指定 NewsArticle 实现 Summary trait。因此，我们仍然可以对 NewsArticle 实例调用 summarize 方法，如下所示：

```rust
use chapter10::{self, NewsArticle, Summary};
let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
```
> 这段代码会打印 New article available! (Read more...)。

>  为 summarize 创建默认实现并不要求对示例 10-13 中 Tweet 上的 Summary 实现做任何改变。其原因是重载一个默认实现的语法与实现没有默认实现的 trait 方法的语法一样。

> 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。如此，trait 可以提供很多有用的功能而只需要实现指定一小部分内容。例如，我们可以定义 Summary trait，使其具有一个需要实现的 summarize_author 方法，然后定义一个 summarize 方法，此方法的默认实现调用 summarize_author 方法：

```rust
pub trait summary {
    fn sumarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read mor from {}...", self.summarize_author())
    }
}
```
> 为了使用这个版本的 Summary，只需在实现 trait 时定义 summarize_author 即可：

```rust
impl Summary for Tweet {
    fn sumarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

```
> 一旦定义了 summarize_author，我们就可以对 Tweet 结构体的实例调用 summarize 了，而 summarize 的默认实现会调用我们提供的 summarize_author 定义。因为实现了 summarize_author，Summary trait 就提供了 summarize 方法的功能，且无需编写更多的代码。

```rust
let tweet = {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```
> 这会打印出 1 new tweet: (Read more from @horse_ebooks...)。

> 注意无法从相同方法的重载实现中调用默认方法。

### trait 作为参数
> 知道了如何定义 trait 和在类型上实现这些 trait 之后，我们可以探索一下如何使用 trait 来接受多种不同类型的参数。

> 例如在示例 10-13 中为 NewsArticle 和 Tweet 类型实现了 Summary trait。我们可以定义一个函数 notify 来调用其参数 item 上的 summarize 方法，该参数是实现了 Summary trait 的某种类型。为此可以使用 impl Trait 语法，像这样：

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
> 对于 item 参数，我们指定了 impl 关键字和 trait 名称，而不是具体的类型。该参数支持任何实现了指定 trait 的类型。在 notify 函数体中，可以调用任何来自 Summary trait 的方法，比如 summarize。我们可以传递任何 NewsArticle 或 Tweet 的实例来调用 notify。任何用其它如 String 或 i32 的类型调用该函数的代码都不能编译，因为它们没有实现 Summary。

### Trait Bound 语法
> impl Trait 语法适用于直观的例子，它实际上是一种较长形式语法的语法糖。我们称为 trait bound，它看起来像：

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
> 这与之前的例子相同，不过稍微冗长了一些。trait bound 与泛型参数声明在一起，位于尖括号中的冒号后面。

> impl Trait 很方便，适用于短小的例子。trait bound 则适用于更复杂的场景。例如，可以获取两个实现了 Summary 的参数。使用 impl Trait 的语法看起来像这样：

```rust
pub fn notify(item: &impl Summary, item2: &impl summary) {
    
}
```


