# Notes
- https://doc.rust-lang.org/book/title-page.html
- https://rustwiki.org/docs/
- https://github.com/rust-lang-cn
- https://actix.rs/  actix A powerful, pragmatic, and extremely fast web framework for Rust
```
The Rust Programming Language
Foreword
Introduction
1. Getting Started
    1.1. Installation
    1.2. Hello, World!
    1.3. Hello, Cargo!
2. Programming a Guessing Game
    3. Common Programming Concepts
    3.1. Variables and Mutability
    3.2. Data Types
    3.3. Functions
    3.4. Comments
    3.5. Control Flow
4. Understanding Ownership
    4.1. What is Ownership?
    4.2. References and Borrowing
    4.3. The Slice Type
5. Using Structs to Structure Related Data
    5.1. Defining and Instantiating Structs
    5.2. An Example Program Using Structs
    5.3. Method Syntax
6. Enums and Pattern Matching
    6.1. Defining an Enum
    6.2. The match Control Flow Operator
    6.3. Concise Control Flow with if let
7. Managing Growing Projects with Packages, Crates, and Modules
    7.1. Packages and Crates
    7.2. Defining Modules to Control Scope and Privacy
    7.3. Paths for Referring to an Item in the Module Tree
    7.4. Bringing Paths Into Scope with the use Keyword
    7.5. Separating Modules into Different Files
8. Common Collections
    8.1. Storing Lists of Values with Vectors
    8.2. Storing UTF-8 Encoded Text with Strings
    8.3. Storing Keys with Associated Values in Hash Maps
9. Error Handling
    9.1. Unrecoverable Errors with panic!
    9.2. Recoverable Errors with Result
    9.3. To panic! or Not to panic! 
10. Generic Types, Traits, and Lifetimes
    10.1. Generic Data Types
    10.2. Traits: Defining Shared Behavior
    10.3. Validating References with Lifetimes
11. Writing Automated Tests
    11.1. How to Write Tests
    11.2. Controlling How Tests Are Run
    11.3. Test Organization
12. An I/O Project: Building a Command Line Program
    12.1. Accepting Command Line Arguments
    12.2. Reading a File
    12.3. Refactoring to Improve Modularity and Error Handling
    12.4. Developing the Library’s Functionality with Test Driven Development
    12.5. Working with Environment Variables
12.6. Writing Error Messages to Standard Error Instead of Standard Output
    13. Functional Language Features: Iterators and Closures
    13.1. Closures: Anonymous Functions that Can Capture Their Environment
    13.2. Processing a Series of Items with Iterators
    13.3. Improving Our I/O Project
    13.4. Comparing Performance: Loops vs. Iterators
14. More about Cargo and Crates.io
    14.1. Customizing Builds with Release Profiles
    14.2. Publishing a Crate to Crates.io
    14.3. Cargo Workspaces
    14.4. Installing Binaries from Crates.io with cargo install
14.5. Extending Cargo with Custom Commands
    15. Smart Pointers
    15.1. Using Box<T> to Point to Data on the Heap
    15.2. Treating Smart Pointers Like Regular References with the Deref Trait
    15.3. Running Code on Cleanup with the Drop Trait
    15.4. Rc<T>, the Reference Counted Smart Pointer
    15.5. RefCell<T> and the Interior Mutability Pattern
15.6. Reference Cycles Can Leak Memory
    16. Fearless Concurrency
    16.1. Using Threads to Run Code Simultaneously
    16.2. Using Message Passing to Transfer Data Between Threads
    16.3. Shared-State Concurrency
16.4. Extensible Concurrency with the Sync and Send Traits
    17. Object Oriented Programming Features of Rust
    17.1. Characteristics of Object-Oriented Languages
    17.2. Using Trait Objects That Allow for Values of Different Types
17.3. Implementing an Object-Oriented Design Pattern
    18. Patterns and Matching
    18.1. All the Places Patterns Can Be Used
    18.2. Refutability: Whether a Pattern Might Fail to Match
    18.3. Pattern Syntax
19. Advanced Features
    19.1. Unsafe Rust
    19.2. Advanced Traits
    19.3. Advanced Types
    19.4. Advanced Functions and Closures
19.5. Macros
    20. Final Project: Building a Multithreaded Web Server
    20.1. Building a Single-Threaded Web Server
    20.2. Turning Our Single-Threaded Server into a Multithreaded Server
    20.3. Graceful Shutdown and Cleanup
21. Appendix
    21.1. A - Keywords
    21.2. B - Operators and Symbols
    21.3. C - Derivable Traits
    21.4. D - Useful Development Tools
    21.5. E - Editions
    21.6. F - Translations of the Book
    21.7. G - How Rust is Made and “Nightly Rust”

```

# Rust Mac install error
- error
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs --no-modify-path | sh

# output
error: could not amend shell profile: '/Users/xxxx/.bashrc': could not write rcfile file: '/Users/xxxx/.bashrc': Permission denied (os error 13)
```

- Solution
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path
```


# 中文笔记结构目录
```
Rust 程序设计语言
前言
简介
1. 入门指南
    1.1. 安装
    1.2. Hello, World!
    1.3. Hello, Cargo!
2. 猜数字游戏
3. 通用编程概念
    3.1. 变量和可变性
    3.2. 数据类型
    3.3. 函数
    3.4. 注释
    3.5. 控制流
4. 认识所有权
    4.1. 什么是所有权？
    4.2. 引用和借用
    4.3. 切片 slice
5. 使用结构体组织关联数据
    5.1. 定义和举例说明结构体
    5.2. 使用结构体的代码例子
    5.3. 方法语法
6. 枚举和模式匹配
    6.1. 定义枚举
    6.2. match 控制流运算符
    6.3. if let 简单控制流
7. 使用包、Crate 和模块管理不断增长的项目
    7.1. 包和 crate
    7.2. 定义模块来控制作用域与私有性
    7.3. 路径用于引用模块树中的项
    7.4. 使用 use 关键字将名称引入作用域
    7.5. 将模块分割进不同文件
8. 常见集合
    8.1. 使用 vector 存储一列值
    8.2. 使用字符串存储 UTF-8 编码的文本
    8.3. 在哈希 map 中存储键和关联值
9. 错误处理
    9.1. panic! 与不可恢复的错误
    9.2. Result 与可恢复的错误
9.3. panic! 还是不 panic!
10. 泛型、trait 与生命周期
    10.1. 泛型数据类型
    10.2. trait：定义共享的行为
    10.3. 生命周期与引用有效性
11. 编写自动化测试
    11.1. 如何编写测试
    11.2. 测试如何测试运行
11.3. 测试的组织结构
12. 一个 I/O 项目：构建命令行程序
    12.1. 接受命令行参数
    12.2. 读取文件
    12.3. 重构以改进模块化与错误处理
    12.4. 采用测试驱动开发完善库的功能
    12.5. 处理环境变量
12.6. 将错误信息输出到标准错误而不是标准输出
13. Rust 中的函数式语言功能：迭代器与闭包
    13.1. 闭包：可以捕获其环境的匿名函数
    13.2. 使用迭代器处理元素序列
    13.3. 改进之前的 I/O 项目
13.4. 性能比较：循环对迭代器
14. 更多关于 Cargo 和 Crates.io 的内容
    14.1. 采用发布配置自定义构建
    14.2. 将 crate 发布到 Crates.io
    14.3. Cargo 工作空间
    14.4. 使用 cargo install 从 Crates.io 安装二进制文件
14.5. Cargo 自定义扩展命令
15. 智能指针
    15.1. 使用 Box<T> 指向堆上的数据
    15.2. 使用 Deref trait 将智能指针当作常规引用处理
    15.3. 使用 Drop Trait 运行清理代码
    15.4. Rc<T> 引用计数智能指针
    15.5. RefCell<T> 与内部可变性模式
15.6. 引用循环会导致内存泄漏
16. 无畏并发
    16.1. 使用线程同一时间运行代码
    16.2. 使用消息传递在线程间通信
    16.3. 共享状态并发
    16.4. 使用Sync 与 Send Trait 的可扩展并发
17. Rust 的面向对象编程特性
    17.1. 面向对象语言的特点
    17.2. 为使用不同类型的值而设计的 trait 对象
    17.3. 面向对象设计模式的实现
18. 模式和匹配
    18.1. 所有可能会用到模式的位置
    18.2. Refutability（可反驳性）: 模式是否会匹配失效
    18.3. 模式语法
19. 高级特征
    19.1. 不安全的 Rust
    19.2. 高级 trait
    19.3. 高级类型
    19.4. 高级函数与闭包
    19.5. 宏
20. 最后的项目: 构建多线程 Web 服务器
    20.1. 构建单线程 Web 服务器
    20.2. 将单线程服务器变为多线程服务器
    20.3. 优雅停机与清理
21. 附录
    21.1. A - 关键字
    21.2. B - 运算符与符号
    21.3. C - 可派生的 trait
    21.4. D - 实用开发工具
    21.5. E - 版本
    21.6. F - 本书译本
    21.7. G - Rust 是如何开发的与 “Nightly Rust”
```