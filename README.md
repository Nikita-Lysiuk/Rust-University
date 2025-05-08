Lesson 9: Overriding Default Traits in Rust

In this lesson, I explored how Rust allows us to implement and override default traits like `PartialEq`, `PartialOrd`, `Debug`, and others to give custom behavior to our types. Instead of relying on derived implementations, I learned to take full control over how comparison, ordering, and formatting is handled — crucial for game development and engine-level abstractions.

Key Topics:

Why Override Traits:
- Derive works for many types, but sometimes you need logic-specific comparisons (e.g., animation event time + priority).
- Traits like `PartialEq` and `PartialOrd` let you define custom equality and sorting.
- Enables expressive APIs and proper behavior in sorted containers or match expressions.

PartialEq:
- Defines equality comparison `==`.
- Must return a boolean representing logical equality of two values.
Example:
```rust
impl PartialEq for AnimationEvent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.time == other.time &&
        self.importance as u8 == other.importance as u8
    }
}
```

PartialOrd:

- Enables <, >, <=, >= comparisons.

- Should return Some(Ordering) or None for incomparable cases.

- You can chain logic with then_with for multi-field comparison.

Example:

```rust 
impl PartialOrd for AnimationEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.time
                .partial_cmp(&other.time)?
                .then_with(|| (self.importance as u8).cmp(&(other.importance as u8)))
                .then_with(|| self.name.cmp(&other.name))
        )
    }
}
```

Conclusion:
This lesson helped me understand how Rust’s trait system allows fine-grained control over behavior that is often hardcoded in other languages. By overriding default trait logic, I can ensure my types behave exactly as they should — whether they're used in gameplay systems, simulations, or ECS sorting. These skills are foundational for writing robust and expressive systems in modern game engines.