# Rust tutorial
Rust is a new open-source systems programming language created by Mozilla and a community of volunteers, designed to help developers create fast, secure applications which take full advantage of the powerful features of modern multi-core processors.(Mozilla.org)  

## Installing rustup on Linux or macOS
If you are using **Linux** or **macOS**, enter the following command to a terminal:

```
$ curl https://sh.rustup.rs -sSf | sh
```
If the installation is successful, the following line will appear:
```
Rust is installed now. Great!
```
## Installing rustup on Windows
[https://www.rust-lang.org/install.html](https://www.rust-lang.org/install.html)

## Updating and Uninstalling
To update:
```
$ rustup update
```
To uninstall:
```
$ rustup self uninstall
```

## Installing IDE
I prefer using **IntelliJ** over **Visual Studio Code**.  
### IntelliJ Installation 
Download and Install: [https://www.jetbrains.com/idea/](https://www.jetbrains.com/idea/)  
Install Rust plugin: [https://intellij-rust.github.io/](https://intellij-rust.github.io/)

### Visual Studio Code
Download and Install: [https://code.visualstudio.com/](https://code.visualstudio.com/)

  
## Hello Cargo
Cargo is the Rust build system and package manager. Cargo handles every task you can imagine, such as building your code, downloading libraries or dependencies, and building those libraries.  
  
To Check if cargo is installed:
```
$ cargo --version
```
If you see a version number, you have it!

### Creating a project with Cargo
Syntax:
```
cargo new <projectname>
```
Example: 
```
$ cargo new hello_cargo
$ cd cargo
```
The first command creates a new directory call ***hello_cargo***. Navigate into the hello_cargo directory and list the files. You'll see that Cargo has two files and one directory.

```
    
    ├── Cargo.toml              # Config file
    ├── src                     # Source files
    │   ├── main.rs             # rust default main function file
```
Filename Cargo.toml
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```
**[package]** start of package configuration

**name** name of the package,
**version** the version of the package, **authors** a collection of contributors or authors and **edition** indicates which edition should your code be compiled under.  
**[dependencies]** indicated the start of dependencies list

