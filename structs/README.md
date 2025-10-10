
### Summary with Examples

#### 1. Defining a Struct
- Define with `struct` keyword, give it a name and fields with types.
- Example:  

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

---

#### 2. Creating Instances
- Provide values with **key: value** pairs.
- Order of fields doesn’t matter.

```rust
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

- Access/change fields with **dot notation**:

```rust
println!("{}", user1.email);

let mut user1 = user1;
user1.email = String::from("anotheremail@example.com");
```

---

#### 3. Building Structs with Functions
- Functions can return new struct instances:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

- **Field Init Shorthand**: Avoids repetition when names match:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   // same as username: username
        email,      // same as email: email
        sign_in_count: 1,
    }
}
```

---

#### 4. Struct Update Syntax
- Create a new instance from another, updating only selected fields:

Without shorthand:
```rust
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

With shorthand (`..`):
```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

⚠️ **Note on ownership**: fields like `String` are *moved*; after creating `user2`, you can’t use `user1.username` anymore.

---

#### 5. Tuple Structs
- Similar to tuples but given a distinct name/type.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Destructuring
let Point(x, y, z) = origin;
```

---

#### 6. Unit-Like Structs
- No fields at all. Useful for traits or marker types.

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

---

#### 7. Ownership of Struct Data
- Use owned types (`String`) so struct owns its values.

❌ This fails without lifetimes:  
```rust
struct User {
    active: bool,
    username: &str,   // ERROR: needs lifetime specifier
    email: &str,
    sign_in_count: u64,
}
```

✅ Correct:  
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```


### 📊 Rust Data Type Comparison

| Feature                        | **Structs**                                                                                                                                          | **Tuples**                                                                | **Enums**                                                                                 |
|--------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|------------------------------------------------------------------------------------------|
| **Definition style**           | `struct User { username: String, active: bool }`                                                                                                     | `let tup: (i32, bool, f64) = (500, true, 6.4);`                           | `enum IpAddr { V4(String), V6(String) }`                                                 |
| **Field names**                | ✅ Named (`username`, `active`) → improves readability                                                                                               | ❌ Positional only (`.0`, `.1`, `.2`)                                      | ✅ Each variant can have a name; may also hold data (like mini-structs or tuples).        |
| **Access syntax**              | `user.username`                                                                                                                                      | `tup.0` or destructuring                                                   | `match my_ip { IpAddr::V4(addr) => ..., IpAddr::V6(addr) => ... }`                       |
| **Mutability**                 | Entire instance must be mutable to mutate fields (`let mut user = ...; user.username = ...;`)                                                         | Entire tuple must be mutable                                               | Variant chosen at runtime → type-safe Enum matching; fields inside variant mutable if let |
| **Update syntax**              | Uses `..other_instance` to reuse fields (moves data unless `Copy`)                                                                                   | ❌ No update shorthand                                                      | ❌ Not applicable                                                                         |
| **Use cases**                  | Best for modeling related data with descriptive fields (like objects in OOP).                                                                        | Good for grouping quick values (like returning multiple values from a function). | Best for modeling types with different *variants* (state machines, options, results).   |
| **Custom types**                | Each struct defines a new type.                                                                                                                      | Tuples don’t define a distinct type by themselves (but tuple structs do).  | Each enum defines a distinct type and its variants.                                       |
| **Examples**                   | ```rust struct Point { x: i32, y: i32 } let p = Point { x: 5, y: 10 }; ```                                                                           | ```rust let tup = (1, true, "hello"); println!("{}", tup.2); ```           | ```rust enum Shape { Circle(f64), Rectangle(f64,f64) } let c = Shape::Circle(2.0); ```   |
| **Special forms**              | - Tuple Structs: `struct Color(i32,i32,i32);` <br> - Unit-like Structs: `struct AlwaysEqual;`                                                         | N/A                                                                        | Variants can be unit-like, tuple-like, or struct-like.                                    |

---

✅ Quick Takeaways:
- **Structs** → Clear, descriptive data containers.  
- **Tuples** → Quick, lightweight grouping.  
- **Enums** → Alternatives/variants of a type (often with pattern matching).  

---

