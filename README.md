## RUST

The repository is for practicing programming in RUST language and learning it. The repository contains file in rust language

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
  - Exterior scope can be used inside but not outside 
  - Const the value and the type of the variable doesn't change tin the whole program , use capital snake case for this type of the writing, you have to define the type and the value for it.

  ```rust
  const TIME_IN_DAY
  ```

- Data types in rust 

  - The types of the data are scalar and compound 
  - The integers 
    - goes from i8 to i128 with jumps of 2 multiple
  - The unint
    - can't have a sign
    - range same as int
  - The float 
    - f32 and f64
  - The bool
    - take the bool character 
  - The char
    - Takes one character 
  - The string 

- The compound types

  - Tuples 
    - A type of list and immutable 
    - Size must be assigned 
    - The printing is different and default format for printing doesn't work
    - We can make  a tuple mutable by putting mut in the deceleration
  - Arrays
    - Type of the array is defined by the elements in the array 
    - The adding of the array is not possible as new elements new length array 
    - The array can be made mutable by putting mut 

- The prelude includes the functions that are needed to the fundamental functions 

  - we need to include the package or crate and in that module for the functions
  -  Taking input from the console requires crate
  -  We have to create a mutable input to store the input 

- The arithmetician operation 

  - The operation can be performed in the same data type 
  - like i8 and done arithmetic operation with i8, same data type even in the int
  - Bits are also imp as u8 = 255 +  1 is not possible can give memory error
  - The same operation as any other programing language 
  - For Type casting use the as keyword
    - The "as" for typecasting
    - use the _ operator, or as operator

- For the conditions 

  - The comparison happens in the same type always or need to type cast 
  - and is &&, or is || and not is !.
  - if else and else if is used 

- Functions 

  - functions are passed with type of the parameters that is passed 
  - Let statements is a statement can be used for the assignment
  - The return has to be written at the end of the function without a semicolon and the also return type has to be written
  - The return should work as expression 
  - The return statement can be used with semicolon 

- Memory management heap and stack  

  - The values are stored in the stack and the heap 
  - program are optimized to used the stack and sometimes heap 
  - Depends on usage
  - 