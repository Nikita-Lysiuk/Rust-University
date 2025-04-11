## Lesson 7: Structs, Traits, and Type Implementations

In this lesson, I learned how to define and implement custom data types in Rust using `struct`, `impl`, and traits. I also explored how to automatically derive common traits using `#[derive(...)]`, and understood the difference between independent and dependent traits.

### Key Topics:

- **Structs:**
  - Creating custom data types with named fields.
  - Examples: `Rgb`, `Macierz`.

- **Traits and `#[derive(...)]`:**
  - Independently derivable traits: `Debug`, `Clone`, `PartialEq`, `Default`, `Hash`.
  - Dependent traits:  
    - `Eq` (requires `PartialEq`)  
    - `Ord` (requires `PartialOrd` + `Eq`)  
    - `Copy` (requires `Clone`)

- **`impl` Blocks:**
  - Adding methods to types with `impl`.
  - Usage of `self`, `&self`, `&mut self`, `Self`.
  - Factory methods like `Rgb::white()`, `Rgb::gray()`, and utility methods like `invert()` and `intensity()`.

- **Practical Implementations:**
  - **`Rgb` color struct:**
    - Multiple constructors: from 0–255 or percent ranges.
    - Conversion to CMY.
    - Inversion and intensity calculation.
  - **`Macierz` (Matrix) struct:**
    - Custom matrix constructor with fill value.
    - Zero and unit matrix factories.
    - Manual implementation of `PartialEq` for dimension comparison.
    - Element access and matrix addition with validation.

### Exercises Included:

- Implemented a complete `Rgb` struct with multiple constructors and utilities.
- Built a `Macierz` matrix struct with logic for construction, comparison, addition, and value mutation.
- Practiced handling `Option`, trait derivation, and trait bounds.

---

### Conclusion:

This lesson solidified my understanding of **structural programming in Rust**. I learned how to encapsulate data with associated logic, leverage traits for extended behavior, and build clean, idiomatic APIs. These skills are foundational for writing scalable and expressive Rust code in future modules.