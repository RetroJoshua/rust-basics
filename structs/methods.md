### Summary of "Method Syntax" (The Rust Programming Language, Ch. 5.3)

Rust methods are functions defined within an impl block for a type (struct, enum, or trait object). They always take self (by value, by shared reference &self, or by mutable reference &mut self) as the first parameter, enabling method-call syntax (instance.method(...)). Methods help organize behavior with the data they operate on and improve ergonomics through automatic referencing/dereferencing in method calls.

---

#### Key Points with Examples

**1. What a method is**
- Declared with `fn` inside an `impl T` block and take `self` as the first parameter.
- Similar to functions but associated with a specific type.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area()); // Area: 1500
}
```

---

**2. Defining methods with self**
- `self` forms:
  - `&self`: shared borrow, read-only access.
  - `&mut self`: mutable borrow, allows mutation.
  - `self`: takes ownership (less common; used when transforming/consuming the value).

```rust
impl Rectangle {
    // Immutable borrow - read only
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Mutable borrow - can modify
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Takes ownership - consumes self
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 20 };
    println!("Area: {}", rect.area());
    rect.scale(2);
    println!("Scaled: {:?}", rect); // width: 20, height: 40
    let square = rect.into_square(); // rect is moved here
    // rect.area(); // ERROR: rect was moved
}
```

---

**3. Method-call syntax**
- Call via `instance.method(args)`. This is syntactic sugar for passing `self` as the first argument.

```rust
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    
    // Method syntax (preferred)
    rect.area();
    
    // Equivalent function-style call
    Rectangle::area(&rect);
}
```

---

**4. Automatic referencing and dereferencing**
- Rust inserts `&`, `&mut`, or `*` automatically to match a method's receiver type.
- `p.distance(&q)` and `(&p).distance(&q)` behave the same.

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    
    // Both are equivalent - Rust auto-references
    p1.distance(&p2);
    (&p1).distance(&p2);
}
```

---

**5. Methods with additional parameters**
- Methods can take more parameters after `self`, just like normal functions.

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
}
```

---

**6. Getters and name shadowing**
- A method can share a name with a field; parentheses disambiguate (`rect.width()` vs `rect.width`).
- "Getters" (read-only accessors) are not auto-generated; you define them yourself.

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    
    if rect.width() {  // Calls the method
        println!("Width is {}", rect.width);  // Accesses the field
    }
}
```

---

**7. Associated functions (no self)**
- Functions in `impl` without `self` are "associated functions," often used as constructors.
- Called with the `::` syntax (e.g., `Rectangle::square(size)`).

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let sq = Rectangle::square(10);
    let rect = Rectangle::new(20, 30);
    println!("Square: {:?}", sq);
    println!("Rectangle: {:?}", rect);
}
```

---

**8. Multiple impl blocks**
- A type can have several `impl` blocks; they are equivalent to one consolidated block.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Both methods are available on Rectangle
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
}
```

---

**9. Complete example combining concepts**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    // Associated function (constructor)
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
    
    // Method with &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Method with &mut self
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Getter method
    fn width(&self) -> u32 {
        self.width
    }
}

fn main() {
    let mut rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(10);
    
    println!("rect1 area: {}", rect1.area());
    println!("rect1 width: {}", rect1.width());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    rect1.scale(2);
    println!("After scaling: {:?}", rect1);
}
```

---

**Key Takeaways:**
- Methods organize behavior with data through `impl` blocks
- `self` borrowing follows Rust's ownership rules
- Automatic referencing makes method calls ergonomic
- Associated functions (no `self`) are great for constructors
- Multiple `impl` blocks are allowed for the same type