### 🔑 Core Idea
In Rust, *references* let you refer to a value without taking ownership of it.  
This is called **borrowing**, and it enforces memory safety at compile-time.

---

### 1. References `&`
A **reference** is like a pointer, but it guarantees:
- It will always point to a valid value.
- It follows Rust’s **ownership rules**.

Example:
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);   // pass a reference
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to String
    s.len()
} // s goes out of scope here, but it doesn’t drop anything
```
Here, `&s1` means:
- `s1` is **borrowed**, not moved.
- After the function call, `s1` is still valid.

---

### 2. Mutable References `&mut`
You can also **mutably borrow**:
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```
Here:
- `&mut s` means "give a mutable reference to `s`".
- While a mutable reference exists, no other references (mutable or immutable) can exist.

---

### 3. Borrowing Rules 🛡️
Rust enforces these rules:
1. **Any number** of immutable references (`&`) OR  
   **Exactly one** mutable reference (`&mut`) at a time.
2. References must **always be valid** (can’t outlive the data).

✅ Allowed:
```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2); // multiple immutable references are fine
```

❌ Not allowed:
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // error: cannot borrow as mutable while also borrowed as immutable
```

---

### 4. Dangling References (Forbidden in Rust)
Rust prevents references that point to deallocated memory:
```rust
fn dangle() -> &String { // 🚫 this won’t compile
    let s = String::from("hello");
    &s
} // s goes out of scope, ref would dangle
```
Instead, return ownership:
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

---

✅ **Summary**
- `&T` = immutable borrow (read-only).
- `&mut T` = mutable borrow (read/write).
- Borrow checker ensures:  
  → Multiple readers OR one writer, never both.  
  → No dangling references.
