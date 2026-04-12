# CubeCL Early Return Manual

## Overview

In CubeCL, the clearest documented way to stop execution **for the current kernel unit** (for example, the current thread or invocation) is to use `terminate!()`. The public `cubecl` API documentation describes `terminate` as **"Terminate the execution of the kernel for the current unit."** citeturn1search14

CubeCL's macro/parsing documentation also lists `terminate!` as a special construct and explains that it expands through a `return_expand()` path during kernel expansion. This indicates that `terminate!()` is the framework-supported mechanism for kernel-unit early termination. citeturn1search16

At the same time, CubeCL supports ordinary control flow, return types, and return-oriented IR/control-flow handling for `#[cube]` code. The macro/parsing documentation describes return-type analysis for kernel functions, and the optimizer documentation lists `Return` as one of the control-flow forms it handles. citeturn1search16turn1search26

---

## Recommended Rule of Thumb

- Use **`terminate!()`** when you want to exit **the current kernel entrypoint invocation early**. citeturn1search14turn1search16
- Use **ordinary returned values** in helper `#[cube]` functions when you want function-style early exits such as `return value;`. CubeCL documents support for return types in `#[cube]` functions, and official examples show helper functions and trait methods that return values. citeturn1search16turn1search2turn1search23
- If your only goal is to avoid out-of-bounds work, consider a simple **`if` guard** instead of an explicit early termination. The official README example uses this style for elementwise kernels. citeturn1search2

---

## 1. Early Exit in a Kernel Entrypoint

For a kernel entrypoint (for example, a function annotated with `#[cube(launch)]` or `#[cube(launch_unchecked)]`), the most explicit documented early-exit mechanism is `terminate!()`. The reason is that the public API documentation names it specifically for terminating execution of the current unit, and the macro documentation lists it as a first-class special construct. citeturn1search14turn1search16

### Example: Bounds Check With `terminate!()`

```rust
use cubecl::prelude::*;

#[cube(launch_unchecked)]
fn copy_if_in_bounds<F: Float>(input: &Array<F>, output: &mut Array<F>) {
    if ABSOLUTE_POS >= input.len() {
        terminate!();
    }

    output[ABSOLUTE_POS] = input[ABSOLUTE_POS];
}
```

This pattern is appropriate when a unit should do no further work after a condition fails, such as an out-of-range global index or a filtered-out item. The documented semantics are unit-local rather than kernel-global. citeturn1search14

### When to Prefer This Pattern

- Out-of-bounds protection for unchecked launches. citeturn1search14turn1search16
- Fast rejection of elements that should not participate in the rest of the kernel body. citeturn1search14
- Situations where an explicit “stop here” marker makes the kernel easier to read than nested conditionals. citeturn1search16

---

## 2. Guard-Only Style (No Explicit Early Return)

CubeCL's README demonstrates a common alternative pattern: instead of exiting early, perform the work only inside an `if` block such as `if ABSOLUTE_POS < input.len() { ... }`. This is the documented style shown in the elementwise example. citeturn1search2

### Example: Guarded Execution

```rust
use cubecl::prelude::*;

#[cube(launch_unchecked)]
fn copy_guarded<F: Float>(input: &Array<F>, output: &mut Array<F>) {
    if ABSOLUTE_POS < input.len() {
        output[ABSOLUTE_POS] = input[ABSOLUTE_POS];
    }
}
```

### When to Prefer This Pattern

- Simple elementwise kernels where only one small block of work must be guarded. citeturn1search2
- Cases where you want to match the style shown in official CubeCL examples. citeturn1search2
- Situations where avoiding an explicit termination macro keeps control flow straightforward. citeturn1search2turn1search16

### `terminate!()` vs Guard-Only Style

Both approaches can prevent invalid memory access when a unit is out of range, but the documented README example favors the guard-only style for straightforward elementwise work, while the API and macro documentation make `terminate!()` the explicit mechanism for unit-local termination. citeturn1search2turn1search14turn1search16

---

## 3. Early Return in Helper `#[cube]` Functions

CubeCL supports `#[cube]` functions with return values. The README includes helper functions such as `gelu_scalar` returning a value, and the `sum_things` example includes trait methods and helper functions returning `F`. The macro documentation also describes return-type analysis for kernel functions. citeturn1search2turn1search23turn1search16

Because CubeCL models return-oriented control flow in its macro and optimization pipeline, using `return value;` inside a helper `#[cube]` function is conceptually aligned with the framework. The optimizer documentation explicitly lists `Return` in the handled control-flow forms. citeturn1search16turn1search26

### Example: Helper Function With Value Return

```rust
use cubecl::prelude::*;

#[cube]
fn clamp_positive<F: Float>(x: F) -> F {
    if x <= F::new(0.0) {
        return F::new(0.0);
    }

    x
}
```

### Practical Guidance

Use this style for **function-level** early exits that return a value, but prefer `terminate!()` for **entrypoint kernel invocation** early termination. This distinction best matches the documented API surface and official examples found during this investigation. citeturn1search14turn1search16turn1search2turn1search23

---

## 4. Choosing the Right Pattern

### Pattern A — `terminate!()` in the Entrypoint

Choose this when the current unit should stop executing the rest of the kernel body immediately. This is the most directly documented early-exit mechanism for kernel units. citeturn1search14turn1search16

```rust
#[cube(launch_unchecked)]
fn kernel_a<F: Float>(input: &Array<F>, output: &mut Array<F>) {
    if ABSOLUTE_POS >= input.len() {
        terminate!();
    }

    output[ABSOLUTE_POS] = input[ABSOLUTE_POS];
}
```

### Pattern B — Guarded Body

Choose this for simple bounds protection and minimal branching complexity, especially when following the style used in CubeCL's README examples. citeturn1search2

```rust
#[cube(launch_unchecked)]
fn kernel_b<F: Float>(input: &Array<F>, output: &mut Array<F>) {
    if ABSOLUTE_POS < input.len() {
        output[ABSOLUTE_POS] = input[ABSOLUTE_POS];
    }
}
```

### Pattern C — Helper Function Returns a Value

Choose this when the early exit is part of a value-producing helper routine rather than termination of the kernel entrypoint itself. CubeCL examples and macro documentation support returned values in `#[cube]` functions. citeturn1search2turn1search23turn1search16

```rust
#[cube]
fn kernel_c_helper<F: Float>(x: F) -> F {
    if x < F::new(0.0) {
        return F::new(0.0);
    }

    x
}
```

---

## 5. Best Practices

1. **Prefer `terminate!()` for entrypoint early termination.** This is the clearest documented mechanism for ending execution of the current unit. citeturn1search14turn1search16
2. **Prefer a simple `if` guard for trivial bounds checks.** This is the style shown in CubeCL's official README example. citeturn1search2
3. **Use value returns in helper `#[cube]` functions when you are expressing function logic, not unit termination.** CubeCL examples and macro parsing support this model. citeturn1search2turn1search23turn1search16
4. **Validate on your actual backend and CubeCL version.** CubeCL supports multiple runtimes/backends, and the README explicitly notes that not all platforms support the same features. citeturn1search2

---

## 6. Summary

If you need an **early return equivalent in a CubeCL kernel entrypoint**, use **`terminate!()`**. That is the documented API-level mechanism for terminating execution of the current unit. citeturn1search14turn1search16

If you only need to skip work for out-of-range indices, a plain **`if` guard** is also a good and officially demonstrated pattern. citeturn1search2

If you are writing a helper `#[cube]` function that computes and returns a value, ordinary **`return value;`** style logic is consistent with CubeCL's documented return-type support and return-oriented control-flow handling. citeturn1search16turn1search23turn1search26
