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
cargo new --lib restaurent   #to make the new library crate, with cargo directory 
```

### Concepts

- Extension of file is .rs 

- Don't have any spaces in the file as the file is saved 

- The main function is executed automatically when program is executed

- Only one line comments are there in rust by "//"

- The block comments are /* and */ 

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
  - The const are never assigned on run time, we can have 100_000 for the value that is acceptable.

  ```rust
  const TIME_IN_DAY
  ```

- Statements define a value and the expression is getting the value 

  - if else can be also used in the expression to assign the value 

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
  - The functions can be used in the snake case naming convention

- Loop 

  - the basic loop is the loop that executes until the break statement is called 
  - While loop is same as the other languages 
  - The for loop uses the iter function for the controlling 
  - and the range can be done by double quotes like 1..10, get number from 1 to 9.

- Memory management heap and stack  

  - The values are stored in the stack and the heap 
  - program are optimized to used the stack and sometimes heap 
  - Depends on usage
  - The ownership rules 
    - each variable has one owner only at a time 
    - the owner goes out of space it will drop the value
    - Scope is like the block of the code that is executed by parenthesis
    - When we assign a value to equal to another value it moves the value rather than copy it
    - For cloning the value use the .clone method to make an another copy of the same string
    - Reference don't take owner ship of the string 
    - Kind of same concept as pointers 
    - Reference are immutable by default 
    - we can change that by making the variable mutable and reference mutable too 

- Structs 

  - The structs are like any other programing language 
  - tuple and structs both are used 
  - The structs are type of the different data types and element accessed by . notation 
  - Tuple structs can be used as well for passing the values in the function 
  - we can reference to values as well from the structs 

- Enums 

  - An enum type is a special data type that enables for a variable to be a set of predefined constants. The variable must be equal to one of the values that have been predefined for it.
  - enum can group the structs as well 
  - all the variables can be grouped together to store in the struct 
  - enum can be used to keep only one variant of the data-type as well 
  - A value can either exists or can be null value for the enum 
  - Some key word is used for store any value or none value 
  - match takes to match the value in the structs 
  - The match will bind to the enum 

- Module sysytem 

  - src has the main files for the rust directory system 
  - lib.rs is in the src directory that has the crate system that can create a library system  for the package
  - A package at-least have one crate 
  - If lib.rs is in the root directory rust will automatically create a library with that directory 
  - mod are keywords used to specify the module in the rust system
  - super keyword can be used to access the parent module 
  - if a module is private we can use the pub keyword to make it available for the other functions to use 
  - enum all the variants are public by default 
  - we can use the "use" keyword to bring the module to the scope 
  - we can also rename to make the conflict go away for the imports by using the as keyword
  - we can also import multiple modules by importing in the dictionary format one_module:{module,module2}
  - use of the self and crate the self is used for relative path and crate is used for absolute path 

- Collections in Rust

  - vector are dynamic array in rust 
  - vectors are similar to arrays but with additional capabilities such as automatic resizing when elements are added or removed.
  - push method is used to append into the vector 
  - we can also take the reference to the vector as well by &vector_name 
  - we can use a for loop for iteration in vector
  - enum can be stored in vectors as well but of the same type 
  - the strings can be used for the storing the bytes 
  - they are stored as utf-8 bytes 
  - strings can be created in multiple ways 
  - they are stored as UTF-8 
  - indexing is not that easy we use bytes method to iterate through the string 
  - graphenses in rust to map the values of the string , we also need to import 
  - hashmap used aslo a crate import and methods can be imported 





**Links** 

```text
https://www.youtube.com/watch?v=VFIOSWy93H0&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=4
```



