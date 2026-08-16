#!/usr/bin/env python3
"""Transform a CubeCL kernel source file into a plain-Rust (rayon backend) one.

The transform is purely mechanical and preserves floating-point operation order
exactly, so the emitted kernel is bit-identical to the CubeCL one wherever the
underlying elementary functions agree. That property is what makes migrating
253,961 generated files by transformation safer than re-deriving them from the
Maple sources: it cannot introduce a translation bug that the CubeCL tree does
not already have.

It is sound because the generated kernel bodies are pure straight-line code.
Surveyed across the whole tree, the bodies contain:

  * zero `for`, zero `while`, zero `else`, zero `return`, zero `let mut`
  * zero direct `select(...)` calls
  * exactly one `if`, the `ip < zk.len()` (or `vrho.len()`) bounds guard

so there is no control flow to reason about beyond the guard itself.

Rules applied:

  1. drop `use cubecl::prelude::*;` and the `#[cube...]` attributes
  2. `&Array<f64>` -> `&[f64]`, `&mut Array<f64>` -> `&mut [f64]`
  3. `let ip = ABSOLUTE_POS; if ip < G.len() { ... }` -> `for ip in 0..G.len() { ... }`
     (brace structure is unchanged, so the tail of the file is untouched)
  4. strip the `::<f64>` turbofish from math-helper calls
  5. repoint `libxc_kernel_math::` at the plain-Rust math crate

Note on `f64::ln` / `sqrt` / `exp` / `powf` / ...: under `#[cube]` these lowered
to per-backend intrinsics, which is why the HIP leg diverged from the CPU leg by
~1e-15. As plain Rust they resolve to the system libm -- the same libm libxc
itself calls -- so this backend should sit closer to the oracle, not further.
"""
from __future__ import annotations

import re

MATH_CRATE = "libxc_rkernel_math"
CRATE_PREFIX = "libxc_rkernel_"

_CUBE_ATTR = re.compile(r"^#\[cube(?:\([^)]*\))?\]\n", re.M)
_GUARD = re.compile(
    r"[ \t]*let ip = ABSOLUTE_POS;\n[ \t]*if ip < (\w+)\.len\(\) \{"
)
_TURBOFISH = re.compile(r"\b([A-Za-z_][A-Za-z_0-9]*)::<f64>")
_GENERIC_PARAM = re.compile(r"<\s*F\s*:\s*Float\s*>")
_BARE_F = re.compile(r"\bF\b")
_CAST_FROM = "F::cast_from("
_FLOAT_LITERAL = re.compile(r"-?\d+\.\d*(?:[eE][-+]?\d+)?(?:_f\d+)?|-?\d+\.\d*|-?\d+_f\d+")


def _strip_cast_from(src: str) -> str:
    """Remove `F::cast_from(EXPR)` wrappers, keeping EXPR.

    With `F` resolved to `f64` the cast is the identity, so the wrapper is pure
    noise -- and it is the single most common construct in the tree (719,906
    occurrences). Parens are matched by depth rather than by regex because the
    argument can itself contain calls and nested casts.
    """
    out: list[str] = []
    i = 0
    while True:
        j = src.find(_CAST_FROM, i)
        if j < 0:
            out.append(src[i:])
            return "".join(out)
        out.append(src[i:j])
        k = j + len(_CAST_FROM)
        start = k
        depth = 1
        while k < len(src) and depth:
            c = src[k]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
            k += 1
        if depth:
            raise UnsupportedKernel("unbalanced parens in F::cast_from(...)")
        inner = src[start : k - 1]
        if _CAST_FROM in inner:
            inner = _strip_cast_from(inner)
        # `F::cast_from` is the identity only when its argument is already f64.
        # For a float literal that is always true, so the wrapper just goes
        # away. Anything else (an integer, an index, a variable) may be a real
        # conversion, so keep it as an explicit cast -- `as f64` on an f64 is a
        # no-op, making this safe in both cases.
        out.append(inner if _FLOAT_LITERAL.fullmatch(inner.strip()) else f"({inner} as f64)")
        i = k


def _split_top_level(src: str) -> list[str]:
    """Split on commas that sit at paren/bracket depth zero."""
    parts, depth, cur = [], 0, []
    for c in src:
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        if c == "," and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(c)
    parts.append("".join(cur))
    return parts


def _comment_mask(src: str) -> list[bool]:
    """True at every index that sits inside a comment or string literal.

    Needed because the math modules document themselves with prose like
    "uses nested `select()` calls", and a scanner that does not skip comments
    tries to rewrite that as a call with zero arguments.
    """
    mask = [False] * len(src)
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                mask[k] = True
            i = j
        elif c == "/" and i + 1 < n and src[i + 1] == "*":
            j = src.find("*/", i + 2)
            j = n if j < 0 else j + 2
            for k in range(i, j):
                mask[k] = True
            i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j = min(j + 1, n)
            for k in range(i, j):
                mask[k] = True
            i = j
        else:
            i += 1
    return mask


def resolve_select(src: str) -> str:
    """Rewrite CubeCL `select(c, a, b)` as `(if c { a } else { b })`.

    Faithful, not a relaxation: both arms are already-evaluated expressions at
    the call site in this codebase, so eager `select` and lazy `if/else` do the
    same work in the same order. Calls nest, so arguments are split on
    depth-zero commas and each arm is rewritten recursively.
    """
    needle = "select("
    mask = _comment_mask(src)
    out: list[str] = []
    i = 0
    while True:
        j = src.find(needle, i)
        # Only a real call: not the tail of an identifier like `my_select(`,
        # and not prose inside a doc comment.
        while j > 0 and (
            src[j - 1].isalnum() or src[j - 1] == "_" or mask[j]
        ):
            j = src.find(needle, j + 1)
        if j < 0:
            out.append(src[i:])
            return "".join(out)
        out.append(src[i:j])
        k = j + len(needle)
        start = k
        depth = 1
        while k < len(src) and depth:
            c = src[k]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
            k += 1
        if depth:
            raise UnsupportedKernel("unbalanced parens in select(...)")
        args = _split_top_level(src[start : k - 1])
        # Multi-line calls carry a trailing comma, which yields a 4th empty arg.
        if args and not args[-1].strip():
            args.pop()
        if len(args) != 3:
            raise UnsupportedKernel(f"select() with {len(args)} args, expected 3")
        cond, a, b = (resolve_select(x.strip()) for x in args)
        out.append(f"(if {cond} {{ {a} }} else {{ {b} }})")
        i = k


def _resolve_generics(src: str) -> str:
    """Turn `fn f<F: Float>(x: F) -> (F, F)` into `fn f(x: f64) -> (f64, f64)`.

    The CSE chunk helpers are generic scalar functions; every instantiation in
    the tree is at `f64` (the kernels are f64-concrete by design), so the
    generic parameter is resolved rather than carried.
    """
    src = _strip_cast_from(src)
    # Collect the structs whose generic parameter is about to be dropped, so
    # their *uses* (`-> Chunk0Out<F>`) can be de-genericised too. Scoped to
    # names declared in this file: a blanket `\w+<f64>` -> `\w+` rule would
    # also mangle legitimate generic types such as `Vec<f64>`.
    local_structs = re.findall(r"\bstruct\s+(\w+)\s*<\s*F\s*:\s*Float\s*>", src)
    src = _GENERIC_PARAM.sub("", src)
    for name in set(local_structs):
        src = re.sub(rf"\b{re.escape(name)}\s*<\s*F\s*>", name, src)
    # Replace the bare type `F`, but leave doc comments alone so headers stay
    # readable rather than turning into `<f64: Float>` nonsense.
    lines = src.split("\n")
    for idx, line in enumerate(lines):
        if line.lstrip().startswith("//"):
            continue
        lines[idx] = _BARE_F.sub("f64", line)
    return "\n".join(lines)


class UnsupportedKernel(RuntimeError):
    """The source uses a construct the mechanical transform does not cover."""


def transform_kernel(src: str, *, math_crate: str = MATH_CRATE) -> str:
    """Return the plain-Rust form of a `#[cube]` kernel source file."""
    original = src

    # 4/5 first: they are context-free string rewrites.
    src = src.replace("use cubecl::prelude::*;\n", "")
    # Rename every sibling kernel crate, not just math: some functionals were
    # split across companion crates (e.g. mgga_c_tpssloc_p0..p6) to get under
    # cubecl-macros' memory ceiling, and their wrappers import across crates.
    src = src.replace("libxc_kernel_", CRATE_PREFIX)

    # 1. attributes
    src = _CUBE_ATTR.sub("", src)

    # 1b. `#[derive(CubeType)]` marks a struct that crosses a `#[cube]`
    #     boundary (the chunk-first struct interface, used by gga_c_pbe and
    #     mgga_x_pbe_gx). As plain Rust the struct needs no special derive;
    #     Copy is both valid -- every field resolves to f64 -- and useful,
    #     since these are returned by value from chunk helpers.
    src = src.replace("#[derive(CubeType)]", "#[derive(Clone, Copy)]")

    # 2. array types -> slices
    src = src.replace("&mut Array<f64>", "&mut [f64]")
    src = src.replace("&Array<f64>", "&[f64]")

    # 3. thread-index guard -> loop over the grid
    src, n_guards = _GUARD.subn(
        lambda m: f"    for ip in 0..{m.group(1)}.len() {{", src
    )

    # 4. resolve `<F: Float>` scalar helpers (the CSE chunk functions) to f64.
    #    Must precede the turbofish strip: inside a generic helper the calls read
    #    `pow_1_3::<F>(..)`, and resolving F first turns those into `::<f64>` so
    #    the next step can remove them. Doing it the other way round leaves a
    #    freshly-created `::<f64>` behind on every such call.
    src = _resolve_generics(src)

    # 5. helper turbofish
    src = _TURBOFISH.sub(r"\1", src)

    # ---- validate the result rather than trusting the rewrite -------------
    if "ABSOLUTE_POS" in src:
        raise UnsupportedKernel("ABSOLUTE_POS survived the guard rewrite")
    if "Array<" in src or "cubecl" in src:
        raise UnsupportedKernel("CubeCL types or imports survived")
    if "#[cube" in src:
        raise UnsupportedKernel("a #[cube] attribute survived")
    if "CubeType" in src:
        raise UnsupportedKernel("a CubeType derive survived")
    if "libxc_kernel_" in src:
        raise UnsupportedKernel("an old-tree crate reference survived")
    if "F::" in src or "<F:" in src:
        raise UnsupportedKernel("a generic `F` construct survived")
    if n_guards == 0 and "ABSOLUTE_POS" in original:
        raise UnsupportedKernel("kernel had ABSOLUTE_POS but no recognised guard")
    for banned in (" select(", "=select(", "\tselect("):
        if banned in src:
            raise UnsupportedKernel("direct select() call needs a manual branch")

    src = _rewrite_header(src)
    return src


def _rewrite_header(src: str) -> str:
    """Retitle the module doc comment so it does not claim to be CubeCL."""
    return src.replace(
        "kernel.\n//!\n//! Auto-translated from",
        "kernel (rayon backend).\n//!\n//! Auto-translated from",
    ).replace(
        "//! Preserves exact maple2c variable names and FP operation order.",
        "//! Preserves exact maple2c variable names and FP operation order.\n"
        "//! Mechanically converted from the CubeCL form by "
        "tools/translate_rayon/xform.py.",
    )


def kernel_signature(src: str) -> tuple[str, list[tuple[str, str]], str]:
    """Return (fn_name, [(param, type)], guard_array) for a transformed kernel."""
    m = re.search(r"pub fn (\w+)\(\n(.*?)\n\) \{", src, re.S)
    if not m:
        raise UnsupportedKernel("could not locate the kernel signature")
    name, body = m.group(1), m.group(2)
    params: list[tuple[str, str]] = []
    for line in body.split("\n"):
        line = line.strip().rstrip(",")
        if not line:
            continue
        pname, ptype = (x.strip() for x in line.split(":", 1))
        params.append((pname, ptype))
    g = re.search(r"for ip in 0\.\.(\w+)\.len\(\)", src)
    if not g:
        raise UnsupportedKernel("no grid loop found after transform")
    return name, params, g.group(1)


def transform_math(src: str, *, math_crate: str = MATH_CRATE) -> str:
    """Transform a `#[cube]` math-helper module into plain Rust.

    Same rewrites as `transform_kernel`, plus `select(..)` resolution, and with
    the `#[cfg(test)]` block dropped: those tests drive kernels through a
    CubeCL client and do not carry over.
    """
    marker = "#[cfg(test)]"
    if marker in src:
        src = src[: src.index(marker)].rstrip() + "\n"
    src = src.replace("use cubecl::prelude::*;\n", "")
    src = src.replace("libxc_kernel_", CRATE_PREFIX)
    src = _CUBE_ATTR.sub("", src)
    # `#[comptime] n: usize` marks a value CubeCL must know at kernel-build
    # time. Plain Rust has no such distinction, so it becomes an ordinary
    # runtime parameter.
    src = src.replace("#[comptime] ", "").replace("#[comptime]\n", "")
    src = resolve_select(src)
    src = _resolve_generics(src)
    # After F -> f64: these modules declare `&Array<F>`, so the slice rewrite
    # only matches once the generic has been resolved.
    src = src.replace("&mut Array<f64>", "&mut [f64]")
    src = src.replace("&Array<f64>", "&[f64]")
    src = _TURBOFISH.sub(r"\1", src)
    # Validate on a comment-blanked copy throughout: these modules document
    # themselves with prose like "uses branchless `select()`" and "CubeCL does
    # not support `return` in `#[cube]` functions", none of which is code.
    blanked = "".join(" " if m else c for c, m in zip(src, _comment_mask(src)))
    for bad in ("F::", "<F:", "cubecl", "#[cube", "Array<", "#[comptime", "CubeType"):
        if bad in blanked:
            raise UnsupportedKernel(
                f"a CubeCL construct survived the math transform: {bad!r}"
            )
    if re.search(r"\bselect\s*\(", blanked):
        raise UnsupportedKernel("a select() call survived")
    return src
