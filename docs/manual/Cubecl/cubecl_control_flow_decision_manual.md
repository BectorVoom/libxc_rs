# CubeCL Control-Flow Decision Manual

## Choosing Between `select()`, `if`-return, and `terminate!()`

## Overview

This manual provides a **general decision framework** for choosing between `select()`, `if`-based value flow, and `terminate!()` in CubeCL code. It is written as a **general guide**, not as a project-specific audit, so it focuses on common patterns that appear in helper math functions and kernel entrypoints. The most important documented facts are: **`select()` evaluates both branches and then selects one value**, while **`terminate!()` terminates execution of the current kernel unit**. The official CubeCL examples also show that simple entrypoint bounds protection is commonly written as an **`if` guard** around the work body. citeturn4search51turn4search45turn4search46turn4search54

---

## 1. Documented Semantics You Should Start From

### 1.1 `select()`

CubeCL documents `select()` as follows:

- It **executes both branches**, then selects one value based on the condition. citeturn4search51
- It is **intended to be branchless**, but the final outcome may still depend on the compiler/backend. citeturn4search51
- Because both branches are evaluated, **both branches must always be valid**, and **illegal memory accesses must not occur in either branch**. citeturn4search51

### 1.2 `terminate!()`

CubeCL documents `terminate!()` as a macro that **terminates execution of the kernel for the current unit**. This is a unit-local execution stop, not a value-selection primitive. citeturn4search45turn4search48

### 1.3 Entry-point guard style

In the official CubeCL examples, a common elementwise entrypoint pattern is:

```rust
if ABSOLUTE_POS < input.len() {
    // do work
}
```

This means the **documented, example-backed default style for simple bounds protection** is often a plain `if` guard in the kernel entrypoint, rather than unconditional computation or mandatory use of `terminate!()`. citeturn4search46turn4search54

### 1.4 Safety context for unchecked launch

CubeCL documents `launch_unchecked` with a strict safety contract: the kernel must not perform **out-of-bounds reads or writes**, and must not contain **non-terminating loops**. This makes early reject logic at the entrypoint especially important whenever a unit may be out of range. citeturn4search40

---

## 2. Core Design Principle

You should first decide **what semantic role your code is playing**:

- If the code is **choosing a value**, then the main comparison is usually **`select()` vs `if`/`else` returning a value**. citeturn4search51turn4search46turn4search54
- If the code is **stopping the current kernel unit from doing any more work**, then the main comparison is usually **entrypoint `if` guard vs `terminate!()`**. citeturn4search45turn4search48turn4search46turn4search54

This distinction matters because **`terminate!()` is not a drop-in replacement for helper-function early return**. A helper math function usually wants to **produce a value**; `terminate!()` instead means **stop execution for this unit**. citeturn4search45turn4search48turn4search46turn4search54

---

## 3. Decision Table

## 3.1 Functions that are usually good candidates to keep as `select()`

These patterns usually fit `select()` well because the function is fundamentally **value-selecting**, and both candidate expressions are **safe** and **reasonably cheap** to evaluate. Since CubeCL explicitly states that both branches are evaluated, this category is only appropriate when evaluating both sides is acceptable. citeturn4search51

### Typical examples

- **Absolute-value-like helpers**: choose between `x` and `-x`. citeturn4search51
- **Clamp / saturation helpers**: choose between a bounded result and a constant. citeturn4search51
- **Sign or threshold corrections**: two short algebraic expressions, both always valid. citeturn4search51
- **Simple masks**: choose between a computed value and zero, provided neither side performs invalid memory access. citeturn4search51

### Why `select()` is usually appropriate here

- The function’s job is to **return one of two values**, not to stop the whole kernel unit. citeturn4search45turn4search48
- Both branches are safe to evaluate, matching the documented requirements of `select()`. citeturn4search51
- The official CubeCL model supports helper functions that return values, as shown by helper routines such as `gelu_scalar` in the examples. citeturn4search46turn4search54

### Rule of thumb

Keep `select()` when **all of the following are true**:

1. Both branches are always safe. citeturn4search51
2. Both branches are relatively cheap. citeturn4search51
3. The function is conceptually about **value selection**, not **unit termination**. citeturn4search45turn4search48

---

## 3.2 Functions where `if`-return (or `if`/`else` value flow) should be reconsidered

These patterns are still **value-producing helper functions**, but they may not be ideal for `select()` because CubeCL documents that `select()` evaluates **all branches**. If each branch is expensive, `select()` can make the kernel pay for computations that are never actually needed in the final result. citeturn4search51

### Typical examples

- **Piecewise special-function approximations** where different input regions use different heavy formulas. citeturn4search51
- **Nested region dispatch** where several candidate results are built and then selected. citeturn4search51
- **Iterative helpers** where one path performs many steps, but another path is trivial. citeturn4search51
- **Rare-path expensive logic** where one uncommon branch still gets evaluated because `select()` evaluates both sides. citeturn4search51

### Why `if`-return is worth reconsidering here

- The function still wants to **return a value**, so `terminate!()` is usually the wrong semantic tool. citeturn4search45turn4search48
- However, `select()` may force evaluation of **too much expensive work** because both branches are evaluated. citeturn4search51
- CubeCL examples clearly support helper functions that return values, so value-oriented control flow remains a natural design option. citeturn4search46turn4search54

### Rule of thumb

Reconsider `if`-return when **all or most of the following are true**:

1. The function is still a **value-producing helper**. citeturn4search46turn4search54
2. Different branches have **very different cost**. citeturn4search51
3. Computing every branch would be noticeably wasteful. citeturn4search51
4. Every branch is not required for correctness—only one result is actually needed. citeturn4search51

### Important caution

This category is **not** the same as “replace `select()` with `terminate!()`.” The correct comparison here is usually **`select()` vs `if`/`else` value flow**, because the function’s purpose is still to produce a result. citeturn4search45turn4search48turn4search51

---

## 3.3 Functions or kernels where `terminate!()` should be reconsidered

This category usually applies to **kernel entrypoints**, not ordinary helper math functions. CubeCL documents `terminate!()` as a mechanism to **terminate execution for the current unit**, so it is best aligned with cases where the correct semantic meaning is “this unit should stop here.” citeturn4search45turn4search48

### Typical examples

- **Bounds-check rejection** in an entrypoint kernel before any out-of-range unit performs memory access. citeturn4search40turn4search45
- **Participation filters** where some units should do no more work after a predicate fails. citeturn4search45turn4search48
- **Long entrypoints with a cheap early reject condition** near the top. citeturn4search45turn4search48
- **Unchecked-launch kernels** where non-participating units must be kept away from unsafe memory operations. citeturn4search40turn4search45

### Why `terminate!()` is a candidate here

- The documented semantics of `terminate!()` directly match “stop this current unit.” citeturn4search45turn4search48
- `launch_unchecked` requires that there be **no out-of-bounds memory access**, making early reject logic important. citeturn4search40
- This is a control-flow decision about **whether the unit should continue at all**, not a value-selection decision. citeturn4search45turn4search48

### But do not forget the simpler default

The official CubeCL examples show that a **plain entrypoint `if` guard** is already a strong, documented default for basic bounds protection. Therefore, `terminate!()` should usually be reconsidered only when an **explicit unit stop** expresses the code’s intention more clearly or more locally than a guarded work body. citeturn4search46turn4search54turn4search45

### Rule of thumb

Reconsider `terminate!()` when **all of the following are true**:

1. You are in a **kernel entrypoint**, not just a helper math function. citeturn4search45turn4search48
2. The correct meaning is “this unit should stop doing work now.” citeturn4search45turn4search48
3. The alternative would otherwise expose the unit to invalid or unnecessary work. citeturn4search40turn4search45

---

## 4. Special Red-Flag Cases

### 4.1 Never use `select()` to guard invalid memory access

CubeCL explicitly states that `select()` evaluates both branches, and that both branches must be valid. Therefore, `select()` must **not** be used as a bounds-check substitute around loads or stores that are only valid in one branch. citeturn4search51

### 4.2 Do not use `terminate!()` just to mimic helper-function early return

If a function’s purpose is “compute and return a value,” then `terminate!()` usually changes the semantics too much, because it means “stop execution for this unit,” not “return one of several values from this helper.” citeturn4search45turn4search48turn4search46turn4search54

### 4.3 Do not assume `select()` is always the fastest option

CubeCL documents `select()` as something that **should be branchless**, but also notes that the final behavior may depend on the compiler/backend. This means `select()` should be treated as a useful abstraction, not as an unconditional performance guarantee. citeturn4search51

---

## 5. Practical Review Checklist

Use the following checklist during code review.

### Keep as `select()` if...

- The code is selecting between **two or more values**, not stopping the unit. citeturn4search45turn4search48
- Every candidate branch is always safe to evaluate. citeturn4search51
- The branches are short enough that evaluating all of them is acceptable. citeturn4search51

### Reconsider as `if`-return if...

- The function is still value-producing. citeturn4search46turn4search54
- Some branches are much heavier than others. citeturn4search51
- You are spending real work on branches whose results are usually discarded. citeturn4search51

### Reconsider as `terminate!()` if...

- The code is in a kernel entrypoint. citeturn4search45turn4search48
- A failed condition means the current unit should stop entirely. citeturn4search45turn4search48
- The purpose is not “pick a value,” but “reject this unit from further work.” citeturn4search45turn4search48
- You are protecting an unchecked launch from units that must not proceed. citeturn4search40

---

## 6. Summary

The safest general rule is this:

- **Keep `select()`** for **cheap, always-safe value selection**. citeturn4search51
- **Reconsider `if`-return** for **expensive value-producing branch logic** where `select()` would compute too much unnecessary work. citeturn4search51turn4search46turn4search54
- **Reconsider `terminate!()`** mainly in **kernel entrypoints** where the right semantic meaning is **unit rejection / unit stop**, especially under unchecked launch safety constraints. citeturn4search45turn4search48turn4search40

In short, **`select()` is a value-selection tool**, while **`terminate!()` is a unit-termination tool**. Most confusion disappears once that distinction is kept explicit. citeturn4search51turn4search45turn4search48

---

## Sources

- CubeCL docs.rs: `select` function documentation  
  - https://docs.rs/cubecl/latest/cubecl/frontend/fn.select.html citeturn4search51
- CubeCL docs.rs: crate-level macro listing including `terminate`  
  - https://docs.rs/cubecl/latest/cubecl/ citeturn4search45
- CubeCL docs.rs: `cubecl_macros` crate documentation for `terminate`  
  - https://docs.rs/cubecl-macros/latest/cubecl_macros/ citeturn4search48
- CubeCL docs.rs: `KernelLauncher::launch_unchecked` safety docs  
  - https://docs.rs/cubecl/latest/cubecl/prelude/struct.KernelLauncher.html citeturn4search40
- CubeCL GitHub README / examples showing `if ABSOLUTE_POS < input.len()` guard style and helper return-value structure  
  - https://github.com/tracel-ai/cubecl citeturn4search46
- CubeCL docs.rs crate overview showing the same example pattern  
  - https://docs.rs/crate/cubecl/latest citeturn4search54
