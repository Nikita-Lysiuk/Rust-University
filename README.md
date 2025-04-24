## Lesson 8: Enums as Safe Unions and Pattern Matching in Rust

In this lesson, I learned how Rust's `enum` provides a safe, powerful alternative to traditional `union` types found in C and C++. We discussed the dangers of C++-style unions, and how Rust enforces memory and type safety by tagging each variant. I also explored advanced control flow with `match`, `if let`, and `while let` for clean, concise, and expressive code.

### Key Topics:

- **Unions in C++ vs Enums in Rust:**
  - C++ `union` allows multiple data representations in the same memory space, but without type safety.
  - Rust's `enum` acts as a **tagged union**, tracking the active variant and preventing undefined behavior.
  - Each variant can hold different types and be safely matched at runtime.

- **Enum Syntax and Use Cases:**
  - Defining and instantiating custom enums.
  - Using `enum` to model state and encapsulate different behaviors or data (e.g. `Jednostka`, `Count`).
  - Example:
    ```rust
    enum Jednostka {
        Kilogram,
        Gram,
        Sztuka,
    }

    enum Count {
        LiczbaCalkowita(i32),
        LiczbaRzeczywista(f32),
    }
    ```

- **Pattern Matching with `match`:**
  - Exhaustive and expressive control flow tool.
  - Ensures all variants are covered.
  - Ability to destructure and bind values within arms.
  - Example:
    ```rust
    match count {
        Count::LiczbaCalkowita(x) => println!("Integer: {}", x),
        Count::LiczbaRzeczywista(f) => println!("Float: {}", f),
    }
    ```

- **Simplified Matching with `if let`:**
  - Cleaner syntax when matching a single variant.
  - Useful when you only care about one case:
    ```rust
    if let Count::LiczbaCalkowita(x) = count {
        println!("Just the integer: {}", x);
    }
    ```

- **Pattern Matching in Loops (`while let`):**
  - Great for iterating over `Option` or `Result` until a condition is met.
  - Example:
    ```rust
    while let Some(x) = maybe_number.pop() {
        println!("Popped: {}", x);
    }
    ```

### Exercises Included:

- Defined multiple `enum` types modeling real-world states and units.
- Replaced traditional unions with safe enums for strong typing.
- Implemented logic using `match` and `if let` to cleanly extract and handle values.
- Wrote pattern-matching control flows that ensured exhaustive handling of cases.

---

### Conclusion:

This lesson deepened my understanding of **enums as safe union alternatives** and showed me how to **write powerful, expressive control flow in Rust**. I now see enums not just as variants but as a core tool for building safe, state-driven logic with zero undefined behavior — a concept that’s miles ahead of traditional C++-style unions.