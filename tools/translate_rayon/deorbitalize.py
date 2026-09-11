#!/usr/bin/env python3
"""Emit libxc's deorbitalization chain rule as Rust.

libxc evaluates a deorbitalized meta-GGA (SCAN-L, revSCAN-L, r2SCAN-L and
their correlation halves) with `xc_deorbitalize_func` (`deorbitalize_func.c`):
it evaluates a kinetic-energy functional to get `tau = rho * e_ked(rho, sigma,
lapl)`, feeds that `tau` to the base meta-GGA, and then chain-rules every
derivative through it. The chain rule itself is Maple-generated C:
`maple2c/deorbitalize_{1,2,3,4}.c`, one file per derivative order, each a flat
list of assignments

    v2rho2[0] = ked1_v2rho2[0]*mgga_vtau[0] + ... + mgga_v2rho2[0];

followed by one `if(func->nspin == XC_POLARIZED){ ... }` block.

This translates those files operand for operand into
`crates/libxc-eval/src/eval/deorbitalize_gen.rs`, one function per order:

- `ked1_X[i]` / `ked2_X[i]` / `mgga_X[i]` become `k1.X[..]` / `k2.X[..]` /
  `m.X[..]` at point `ip`, and an output `X[i]` becomes the caller's `X` at
  point `ip`; every stride is `Dimensions::mgga(spin)`, because libxc advances
  all three sets of pointers by `internal_counters_mgga_next` with MGGA
  dimensions (the ked and the base are both MGGA-family functionals of the
  same spin).
- An integer literal becomes a float literal. It is always a factor
  (`2*ked1_vrho[0]*...`), and C converts it to `double` before the multiply,
  so `2.0*...` is the same operation. Grouping and operand order are kept
  exactly, so floating-point results are too.

**The translator refuses to guess.** After substitution, anything left that is
not a number, an operator, a parenthesis or a whitespace is an error, and so
is any statement that does not have the `out[i] = expr` shape. The vocabulary
is closed today (products, sums, integer factors); a libxc update that changes
that will fail here rather than be emitted wrong.

Usage: python3 tools/translate_rayon/deorbitalize.py
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
MAPLE = REPO / "libxc-master" / "src" / "maple2c"
OUT = REPO / "crates" / "libxc-eval" / "src" / "eval" / "deorbitalize_gen.rs"

ORDERS = {1: "first", 2: "second", 3: "third", 4: "fourth"}


class Untranslatable(Exception):
    pass


POL = re.compile(r"if\s*\(\s*func->nspin\s*==\s*XC_POLARIZED\s*\)\s*\{(.*)\}\s*$", re.S)
REF = re.compile(r"\b(ked1|ked2|mgga)_([a-z0-9]+)\[(\d+)\]")
LHS = re.compile(r"^([a-z0-9]+)\[(\d+)\]\s*=\s*(.*)$", re.S)
# An integer literal: digits not part of an identifier, a decimal, or an
# exponent. (An exponent would leave an `e` behind, which the identifier check
# below rejects, so it can never be half-translated.)
INT = re.compile(r"(?<![\w.])(\d+)(?![\w.])")
VAR = {"ked1": "k1", "ked2": "k2", "mgga": "m"}


def statements(text: str) -> list[str]:
    return [" ".join(s.split()) for s in text.split(";") if s.strip()]


def translate(stmt: str, strides: set[str], outputs: list[str]) -> str:
    m = LHS.match(stmt)
    if not m:
        raise Untranslatable(f"not an assignment: {stmt[:80]!r}")
    lhs, idx, rhs = m.groups()
    refs: list[str] = []

    def hold(mm: re.Match) -> str:
        who, field, i = mm.groups()
        strides.add(field)
        refs.append(f"{VAR[who]}.{field}[ip * d_{field} + {i}]")
        return "\x01"

    rhs = REF.sub(hold, rhs)
    left = re.findall(r"[A-Za-z_]\w*", rhs)
    if left:
        raise Untranslatable(f"unknown identifiers {sorted(set(left))} in {stmt[:80]!r}")
    rhs = INT.sub(lambda mm: mm.group(1) + ".0", rhs)
    if re.search(r"[^0-9.+\-*() \x01]", rhs):
        raise Untranslatable(f"unexpected token in {stmt[:80]!r}")
    it = iter(refs)
    rhs = re.sub("\x01", lambda _: next(it), rhs)
    strides.add(lhs)
    if lhs not in outputs:
        outputs.append(lhs)
    return f"{lhs}[ip * d_{lhs} + {idx}] = {rhs};"


def emit_order(n: int) -> str:
    path = MAPLE / f"deorbitalize_{n}.c"
    src = path.read_text()
    m = POL.search(src)
    if not m:
        raise Untranslatable(f"{path.name}: no `if(func->nspin == XC_POLARIZED)` block")
    strides: set[str] = set()
    outputs: list[str] = []
    common = [translate(s, strides, outputs) for s in statements(src[: m.start()])]
    pol = [translate(s, strides, outputs) for s in statements(m.group(1))]

    d_lets = "\n".join(f"    let d_{f} = d.{f} as usize;" for f in sorted(strides))
    o_lets = "\n".join(
        f'    let {f} = o.{f}.as_deref_mut().expect("deorbitalize: every field of the order is supplied");'
        for f in outputs)
    body = "\n".join(f"        {s}" for s in common)
    pbody = "\n".join(f"            {s}" for s in pol)
    return f'''
/// The {ORDERS[n]}-derivative chain rule, `maple2c/deorbitalize_{n}.c`,
/// operand for operand.
#[allow(clippy::all, clippy::pedantic)]
pub(super) fn order_{n}(
    np: usize,
    pol: bool,
    d: &Dimensions,
    o: &mut MggaOutput<'_>,
    m: &MggaScratch<'_>,
    k1: &MggaScratch<'_>,
    k2: &MggaScratch<'_>,
) {{
{d_lets}
{o_lets}
    for ip in 0..np {{
{body}
        if pol {{
{pbody}
        }}
    }}
}}
'''


def main() -> int:
    parts = [emit_order(n) for n in ORDERS]
    OUT.write_text(f'''//! libxc's deorbitalization chain rule (`xc_deorbitalize_func`).
//!
//! GENERATED by tools/translate_rayon/deorbitalize.py from
//! `libxc-master/src/maple2c/deorbitalize_{{1,2,3,4}}.c` -- do not hand-edit.
//!
//! `m` is the base meta-GGA evaluated at `tau = rho * e_ked`, `k1` / `k2` the
//! kinetic-energy functional for the spin-up / spin-down density (`k2` is not
//! read unpolarized), `o` the caller's output. Every array is strided by
//! `d = Dimensions::mgga(spin)`. See `super::deorbitalize` for the driver.
#![allow(unused_variables)]

use libxc_core::dims::Dimensions;
use libxc_core::output::MggaOutput;

use crate::eval::workspace::MggaScratch;
{"".join(parts)}''')
    print(f"wrote {OUT.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
