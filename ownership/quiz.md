
 ### 📝 Ownership Quiz 
 #### Q1. Basic Ownership In Rust, when a variable goes **out of scope**, what happens to its value? 
 a) Nothing happens, it stays in memory 
 b) The value is automatically dropped 
 c) The programmer must call a `free` function 
 d) The garbage collector cleans it up --- 
 #### Q2. Move Semantics What happens after this code? 
 ```rust 
 let s1 = String::from("hello"); 
 let s2 = s1; 
 println!("{}", s1); 
 ```
  a) Prints `"hello"` normally 
  b) Prints garbage data 
  c) Compile-time error 
  d) Runtime panic 
  #### Q3. Copy Trait Which of the following **does implement** the `Copy` trait? 
  a) `String` 
  b) `Vec<i32>` 
  c) `i32` 
  d) `String` + `i32` tuple --- 
  #### Q4. Functions What happens to `text` after it’s passed into the function below? 
  ```rust 
  fn main() {
   let text = String::from("Rust");
   takes(text); 
   println!("{}", text);
   } 
  fn takes(s: String) { 
   println!("{}", s);
    }
```
a) Prints `"Rust"` twice 
b) Compile-time error 
c) Prints `"Rust"` then segfaults 
d) Undefined behavior
     
#### Q5. Clone What’s the difference between **`.clone()`** and simple assignment with `=` for a `String`? --- 
#### Q6. Bonus (True/False) - Ownership rules are checked **at compile time** with no runtime cost. 
