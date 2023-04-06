## RUST

The repository is for practicing programming in RUST language and learning it.

### Installation 

```bash
https://www.rust-lang.org/tools/install   #the website has installation commands , just using commanmd line tools
rustc       #to check if rust is installed , run after installation of the rust command
rustc <filename.rs>     #to compile the file and get the binary value
cargo new <project name>   #to create a new project with cargo package manager
cargo build   #to build the package from the cargo package manager, rrun the command in the project folder
cargo run    #to make the project and run the file as well
cargo check   #to check the code is that has errors 
rustfmt <filename.rs>   #to format the rust files
```

### Concepts

- Extension of file is .rs 
- Don't have any spaces in the file as the file is saved 
- The main function is executed automatically when program is executed
- Only one line comments are there in rust by "//"
- Rust makes a binary file or executable 
- Cargo is the rust default package manager 
- The rust files are formated using the build in rust formatter
- Variables, rust is a statistically type language, the variable must have the type at the time of definition, 
  - Assign value or put the value, can not reassign the value of the different kind to a variable of a type
  - let is use to assign variable , let x = 4; 
  - use the mut keyword to make the variable mutable.
  - recreation of the variable is allowed, for let x =5, let x =4.
  - The variable scope can be change as per the scope of the braces 
  - 