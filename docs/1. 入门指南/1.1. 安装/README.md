# 1. 安装

## 1.1 第一步是安装 Rust。我们将通过 rustup 来下载 Rust，这是一个管理 Rust 版本和相关工具的命令行工具。这需要互联网连接才能下载。

## 1.2 在 Linux 或 macOS 上安装 rustup
```shell
# 如果你使用的是 Linux 或 macOS，打开终端并输入下面命令：
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 这个命令将下载一个脚本并开始安装 rustup 工具，此工具将安装 Rust 的最新稳定版本。可能会提示你输入密码。如果安装成功，将出现下面这行：
Rust is installed now. Great!
```
- 在 macOS 上，可运行以下命令获得 C 编译器：
> `$ xcode-select --install`

> Linux 用户一般应按照相应发行版的文档来安装 GCC 或 Clang。例如，如果你使用 Ubuntu，则可安装 build-essential 包。


## 1.3 更新和卸载
- 通过 rustup 安装 Rust 后，更新到最新版本很简单。在 shell 中运行以下更新命令：
```shell
$ rustup update
```
- 要卸载 Rust 和 rustup，在 shell 中运行以下卸载命令：
```shell
$ rustup self uninstall
```

## 1.4 疑难解答
- 要检查是否正确安装了 Rust，可打开 shell 并输入下面这行：
```shell
$ rustc --version
```
- 你应该看到最新发布的稳定版本的版本号、提交哈希值和提交日期，如下所示格式：
- rustc x.y.z (abcabcabc yyyy-mm-dd)




