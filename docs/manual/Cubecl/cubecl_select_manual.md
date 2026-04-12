# CubeCL `select` Manual

## Overview

`select` in CubeCL is a frontend helper used inside CubeCL kernels to choose between two values based on a boolean condition.

```rust
pub fn select<C>(condition: bool, then: C, or_else: C) -> C
where
    C: CubePrimitive
```

In the official documentation, CubeCL describes `select` as a function that **executes both branches and then selects one value based on the condition**. The documentation also notes that this is **intended to be branchless**, although the final generated code can still depend on the backend compiler.

---

## Primary Purpose

Use `select` when you want a **value-level conditional choice** in a CubeCL kernel and you want to express it in a way that is conceptually **branchless**.

Typical use cases:

- choosing the minimum or maximum of two values
- clamping values
- applying masks
- selecting between two already-safe computed values
- avoiding explicit control-flow branching for simple scalar or element-wise value selection

### Good fit

`select` is a good fit when:

1. both candidate expressions are valid to evaluate
2. neither side performs illegal memory access
3. both sides are simple value computations
4. you want a concise conditional value expression

### Poor fit

`select` is **not** a good fit when:

- one branch would read out of bounds
- one branch would write out of bounds
- one branch is only valid under the condition
- you need true control-flow guarding, not just value selection

In those cases, use an `if` statement instead.

---

## Important Safety Rule

This is the most important point when using `select`:

> **Both branches are evaluated regardless of the condition.**

That means both `then` and `or_else` must be valid for every invocation.

### Consequences

- You **must not** use `select` to guard unsafe memory access.
- You **must not** assume that only the chosen side runs.
- You **must ensure** both values can be computed safely for all threads / units.

If one side performs invalid indexing, the code is incorrect even when the condition would seem to prevent that side from being chosen.

---

## Basic Usage

### Example 1: Absolute value

```rust
use cubecl::prelude::*;

#[cube]
fn abs_value(x: f32) -> f32 {
    select(x < 0.0, -x, x)
}
```

### Why this is safe

- `-x` is valid for any `x`
- `x` is valid for any `x`
- both branches are pure value computations

---

## Example 2: Clamp to zero (ReLU-style)

```rust
use cubecl::prelude::*;

#[cube]
fn relu_scalar(x: f32) -> f32 {
    select(x > 0.0, x, 0.0)
}
```

This is a straightforward value-selection pattern.

---

## Example 3: Select between two already-computed values

```rust
use cubecl::prelude::*;

#[cube]
fn choose_value(a: f32, b: f32, use_a: bool) -> f32 {
    let left = a * 2.0;
    let right = b + 1.0;
    select(use_a, left, right)
}
```

This is appropriate because both `left` and `right` are safe to compute.

---

## Unsafe Pattern: Do Not Use `select` for Bounds Protection

### Incorrect

```rust
use cubecl::prelude::*;

#[cube]
fn unsafe_read(input: &Array<f32>, idx: u32) -> f32 {
    select(idx < input.len(), input[idx], 0.0)
}
```

### Why this is wrong

Even if `idx >= input.len()`, CubeCL documents that **both branches are evaluated**. Therefore `input[idx]` may still be evaluated, which can cause illegal memory access.

### Correct approach

```rust
use cubecl::prelude::*;

#[cube]
fn safe_read(input: &Array<f32>, idx: u32) -> f32 {
    if idx < input.len() {
        input[idx]
    } else {
        0.0
    }
}
```

Use real control flow when the purpose is to protect memory access.

---

## `select` vs `if`

### Use `select` when

- you are choosing between two **safe values**
- both sides may be evaluated safely
- the operation is mostly a value expression
- you want branchless-style intent

### Use `if` when

- one side may be invalid unless the condition holds
- memory access must be guarded
- work should only happen on one side
- you need true control-flow semantics

---

## Type Requirements

The documented signature shows that `select` requires the selected value type `C` to implement `CubePrimitive`.

This means it is intended for primitive CubeCL element-like values used in kernel code.

---

## Practical Guidance

When deciding whether to use `select`, ask these questions:

1. Are both expressions always safe to evaluate?
2. Are both expressions just value computations?
3. Am I selecting a value rather than guarding an operation?
4. Would an invalid memory access still exist if both branches ran?

If the answer to question 4 is **yes**, do **not** use `select`.

---

## Best Practices

- Prefer `select` for simple arithmetic or mask-style value choice.
- Keep both branches side-effect free.
- Never use `select` as a substitute for bounds checks around loads or stores.
- Review generated kernels carefully if backend-specific performance matters.
- Treat `select` as a **value-selection primitive**, not a control-flow primitive.

---

## Summary

CubeCL `select` is useful for **branchless-style conditional value selection** inside kernels. Its major advantage is concise expression of choosing one of two values. However, its most important rule is that **both branches are evaluated**, so it must only be used when **both sides are always safe**.

In short:

- **Use `select` for safe value selection**
- **Use `if` for safety-critical branching and bounds protection**

---

## Sources

- CubeCL docs.rs: `cubecl::frontend::select`
  - https://docs.rs/cubecl/latest/cubecl/frontend/fn.select.html
- CubeCL docs.rs: crate overview
  - https://docs.rs/cubecl/latest/cubecl/index.html
- CubeCL docs.rs: frontend overview
  - https://docs.rs/cubecl/latest/cubecl/frontend/index.html
- CubeCL GitHub repository
  - https://github.com/tracel-ai/cubecl
