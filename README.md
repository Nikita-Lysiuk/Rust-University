# Lesson 5: Error Handling in Rust

## Overview
In this lesson, we explored error handling in Rust, focusing on:
- Utilizing `Option<T>` and `Result<T, E>` for safe and structured error management.
- Applying the `?` operator to simplify error propagation and reduce boilerplate code.
- Implementing functions that work with different numeral systems, including binary, octal, decimal, and Roman numerals.

## Key Takeaways
- ✔️ `Option<T>` **for handling optional values** – Ideal for cases where a function may return either a valid value or `None` (e.g., numeral system conversions).
- ✔️ `Result<T, E>` **for explicit error handling** – Enables structured error messages when operations might fail (e.g., parsing numbers, invalid inputs).
- ✔️ **The `?` operator for error propagation** – Streamlines error handling by automatically returning an error when a function fails inside another function.