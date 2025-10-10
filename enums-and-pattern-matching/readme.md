### Defining an Enum — concise summary (from the Rust Book)

#### What an enum is
- Enums let you express that a value is one of several possible variants (mutually exclusive cases).
- Use when a value can be exactly one of a fixed set of possibilities (e.g., IP version 4 or 6).
- Enum variants are namespaced under the enum name and referenced with `::`.

#### Basic enum syntax and example
- Define variants with no data:
```rust
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) { /* ... */ }

fn main() {
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
```

#### Enum instances and namespacing
- Variants are accessed as `EnumName::Variant`.
- Variants of the same enum share one type (useful for functions that accept any variant).

#### Attaching data to enum variants
- Variants can carry data (like tuple fields or named fields). This avoids needing a separate struct to pair a variant with its data.
- Example: put a `String` into each variant:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home     = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```
- Variant constructors are automatically available: `IpAddr::V4(...)` is a function-like constructor.

#### Variants can hold different types/amounts of data
- Each variant can have different shapes and types:
```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // four numeric components
    V6(String),         // a single String
}

fn main() {
    let home     = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

#### When you might have used structs — comparison
- Using a struct plus an enum field:
```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```
- But enums with data are often more concise and expressive because variant + data can be grouped into one type.

#### A richer example: enum with many variant shapes
```rust
enum Message {
    Quit,                        // unit-like
    Move { x: i32, y: i32 },     // named fields (like a struct)
    Write(String),               // single tuple-like field
    ChangeColor(i32, i32, i32),  // tuple-like with multiple values
}
```
- These variants could alternatively be represented by separate structs, but using a single enum gives you one type that can represent any of those message shapes.

#### Defining methods on enums
- You can `impl` methods for enums just like structs:
```rust
enum Message {
    Write(String),
    // other variants...
}

impl Message {
    fn call(&self) {
        // use `self` to inspect/act on the variant & its data
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

#### The Option<T> enum (no nulls in Rust)
- Rust avoids `null` by using the `Option<T>` enum:
```rust
enum Option<T> {
    None,
    Some(T),
}
```
- `Some(T)` holds a value; `None` means absence.
- `Option<T>` is in the prelude — use `Some`/`None` directly.
- Examples:
```rust
let some_number = Some(5);         // Option<i32> inferred
let some_char   = Some('e');       // Option<char>
let absent_number: Option<i32> = None; // type annotation required for None
```

#### Why Option<T> is safer than null
- `Option<T>` and `T` are distinct types; the compiler prevents using an `Option<T>` as if it were definitely a `T`.
- You must explicitly handle `None` before extracting a `T`, avoiding many null-related bugs.

Example of a compile-time error if you mix `T` and `Option<T>`:
```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y; // error: cannot add `Option<i8>` to `i8`
}
```

#### Using enums in control flow
- To act differently depending on a variant and to extract variant data you use pattern matching (`match`) or other `Option`/enum methods (the book introduces `match` for this purpose).
- `Option<T>` also provides many useful methods (e.g., `map`, `unwrap_or`, `and_then`) — check the docs.

#### Key takeaways (quick list)
- Enums express a value that is exactly one of several variants.
- Variants can hold data — scalar, tuples, structs, or other enums.
- Variant names are namespaced under the enum and act like constructors.
- Enums + pattern matching let you model and exhaustively handle possibilities.
- `Option<T>` encodes presence/absence of a value safely (no `null`).

Source: Rust Book — “Defining an Enum”  
[The Rust Programming Language: Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

