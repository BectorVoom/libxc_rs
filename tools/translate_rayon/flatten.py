#!/usr/bin/env python3
"""Flatten the chunk/meta helper fan-out of a split kernel part into one file.

The CubeCL emitter split large kernel bodies into thousands of tiny scalar
helper functions (`partN/chunkK.rs`, `partN/metaM/chunkK.rs`) purely to keep
cubecl-macros' memory use bounded. Plain rustc has no such ceiling, and the
fan-out is expensive: ~77% of the chunk-file bytes are boilerplate (doc
headers, attribute blocks, imports, signatures), and the 231k+ files dominate
both cargo's fs traffic and rustc's per-item overhead.

This module undoes the fan-out mechanically. Each helper call site

    let (t3, t4, t5) = gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk1(t2);

is replaced by the helper's body as a *block expression*

    let (t3, t4, t5) = {
        let pi = (M_PI as f64);
        let t3 = pi * pi;
        let t4 = 1.0_f64 / t3;
        let t5 = t2 * t4;
        (t3, t4, t5)
    };

which reproduces the function call's semantics exactly: the same operations in
the same order (bit-identical results), and the same scoping (helper-local
names stay invisible outside the block, so no shadowing can leak). Because the
splice is scope-preserving, no renaming is ever needed; the only requirement,
asserted per call, is that argument names equal parameter names, which holds
tree-wide by construction of the CSE emitter.

Meta helpers are the same pattern one level up (a scalar fn calling chunks)
and are flattened recursively before being spliced into the part body.
"""
from __future__ import annotations

import re
from pathlib import Path
from typing import Callable

from xform import UnsupportedKernel

_SIG = re.compile(r"pub fn (\w+)\(([^)]*)\)(?:\s*->\s*[^{]+)?\s*\{")
_HELPER_FILE = re.compile(r"^chunk\d+\.rs$")
_HELPER_DIR = re.compile(r"^meta\d+$")
_MOD_DECL = re.compile(r"^\s*(?:pub\s+)?mod (chunk\d+|meta\d+);\s*$")
_USE_HELPER = re.compile(r"^\s*use (?:chunk\d+|meta\d+)::\w+;\s*$")
_USE_MATH = re.compile(r"^use libxc_rkernel_math::.*;$")
_IDENT = re.compile(r"^[A-Za-z_]\w*$")


def _parse_helper(text: str, origin: str) -> tuple[str, list[str], str]:
    """Return (fn_name, param_names, body) of the single helper fn in `text`.

    `body` is the verbatim text between the fn's braces (tail expression
    included), ready to be wrapped in a block.
    """
    m = _SIG.search(text)
    if not m:
        raise UnsupportedKernel(f"{origin}: no helper fn signature found")
    name = m.group(1)
    params: list[str] = []
    for p in m.group(2).split(","):
        p = p.strip()
        if not p:
            continue
        pname, _, ptype = p.partition(":")
        if ptype.strip() != "f64":
            raise UnsupportedKernel(
                f"{origin}: helper param `{p}` is not f64; cannot splice"
            )
        params.append(pname.strip())
    # Balanced-brace scan from the signature's opening brace.
    i = m.end() - 1
    depth = 0
    for j in range(i, len(text)):
        c = text[j]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                body = text[m.end(): j]
                if _SIG.search(text[j:]):
                    raise UnsupportedKernel(
                        f"{origin}: more than one fn in helper file"
                    )
                return name, params, body
    raise UnsupportedKernel(f"{origin}: unbalanced braces in helper fn")


def _reindent(body: str, indent: str) -> str:
    """Shift a helper body (originally indented 4) under `indent`."""
    out: list[str] = []
    for line in body.strip("\n").split("\n"):
        if not line.strip():
            out.append("")
        elif line.startswith("    "):
            out.append(indent + "    " + line[4:])
        else:
            out.append(indent + "    " + line.lstrip())
    return "\n".join(out)


def _splice_calls(mod_text: str, helpers: dict[str, tuple[list[str], str]],
                  origin: str) -> str:
    """Replace every `let X = helper(args);` in `mod_text` with a block."""
    used = dict.fromkeys(helpers, 0)
    out: list[str] = []
    lines = mod_text.split("\n")
    i = 0
    call_re = re.compile(
        r"^(\s*)let (\([^)]*\)|\w+) = (\w+)\(([^()]*)\);\s*$"
    )
    while i < len(lines):
        line = lines[i]
        m = call_re.match(line)
        if not (m and m.group(3) in helpers):
            # A helper call must never span lines or nest in a larger
            # expression; if one does, the emit must fail rather than leave
            # the call dangling after its helper file is dropped.
            for h in helpers:
                if h in line:
                    raise UnsupportedKernel(
                        f"{origin}: unrecognised reference to helper `{h}`: "
                        f"{line.strip()!r}"
                    )
            out.append(line)
            i += 1
            continue
        indent, bind, name, argstr = m.groups()
        params, body = helpers[name]
        args = [a.strip() for a in argstr.split(",") if a.strip()]
        if args != params:
            raise UnsupportedKernel(
                f"{origin}: call args {args} != params {params} for `{name}`"
            )
        for a in args:
            if not _IDENT.match(a):
                raise UnsupportedKernel(
                    f"{origin}: non-identifier argument `{a}` to `{name}`"
                )
        used[name] += 1
        out.append(f"{indent}let {bind} = {{")
        out.append(_reindent(body, indent))
        out.append(f"{indent}}};")
        i += 1
    unused = [h for h, n in used.items() if n == 0]
    if unused:
        raise UnsupportedKernel(
            f"{origin}: helpers never called: {sorted(unused)[:5]}"
        )
    return "\n".join(out)


def _strip_helper_decls(mod_text: str) -> tuple[str, list[str]]:
    """Drop `mod chunkN;`/`use chunkN::f;` lines; keep everything else.

    Returns (text, existing_math_use_lines).
    """
    kept: list[str] = []
    math_uses: list[str] = []
    for line in mod_text.split("\n"):
        if _MOD_DECL.match(line) or _USE_HELPER.match(line):
            continue
        if _USE_MATH.match(line.strip()):
            math_uses.append(line.strip())
        kept.append(line)
    return "\n".join(kept), math_uses


def _merge_math_uses(text: str, have: list[str], extra: set[str]) -> str:
    """Insert math imports used by helpers but missing from the caller."""
    missing = sorted(extra - set(have))
    if not missing:
        return text
    lines = text.split("\n")
    # Insert after the last existing use line (or after the attr block).
    at = 0
    for idx, line in enumerate(lines):
        s = line.strip()
        if s.startswith("use ") or s.startswith("#![") or s.startswith("//!"):
            at = idx + 1
    return "\n".join(lines[:at] + missing + lines[at:])


def flatten_helper_dir(src_dir: Path, transform: Callable[[str], str]) -> str:
    """Flatten a part/meta directory into single-file source text.

    `src_dir` holds `mod.rs` (the caller) plus `chunkN.rs` helper files and/or
    nested `metaM/` helper directories. Every file is passed through
    `transform` (the CubeCL -> plain-Rust rewrite) before splicing, so the
    result is exactly what translate.py would have emitted, minus the fan-out.
    """
    mod_path = src_dir / "mod.rs"
    if not mod_path.is_file():
        raise UnsupportedKernel(f"{src_dir.name}/: no mod.rs")

    helpers: dict[str, tuple[list[str], str]] = {}
    helper_math_uses: set[str] = set()
    entries = sorted(src_dir.iterdir())
    for p in entries:
        if p.is_file() and _HELPER_FILE.match(p.name):
            text = transform(p.read_text())
            name, params, body = _parse_helper(text, f"{src_dir.name}/{p.name}")
            helpers[name] = (params, body)
            helper_math_uses.update(
                l.strip() for l in text.split("\n") if _USE_MATH.match(l.strip())
            )
        elif p.is_dir() and _HELPER_DIR.match(p.name):
            flat = flatten_helper_dir(p, transform)
            name, params, body = _parse_helper(flat, f"{src_dir.name}/{p.name}")
            helpers[name] = (params, body)
            helper_math_uses.update(
                l.strip() for l in flat.split("\n") if _USE_MATH.match(l.strip())
            )
        elif p.name == "mod.rs":
            continue
        else:
            raise UnsupportedKernel(
                f"{src_dir.name}/: unexpected entry `{p.name}`"
            )

    text = transform(mod_path.read_text())
    text, math_uses = _strip_helper_decls(text)
    if helpers:
        text = _splice_calls(text, helpers, src_dir.name)
        text = _merge_math_uses(text, math_uses, helper_math_uses)

    for name in helpers:
        if re.search(rf"\b{re.escape(name)}\s*\(", text):
            raise UnsupportedKernel(
                f"{src_dir.name}/: a call to `{name}` survived flattening"
            )
    # Blanket sweep for anything the per-helper check could miss (e.g. a
    # cross-directory helper reference). Definition lines are masked: when
    # this dir is itself a meta, its own `pub fn ..._metaN(` must not trip
    # the pattern.
    blanked = "\n".join(
        "" if "pub fn " in line else line for line in text.split("\n")
    )
    for pat in (r"\b\w+_chunk\d+\s*\(", r"\b\w+_meta\d+\s*\(",
                r"\bmod (?:chunk|meta)\d+;"):
        if re.search(pat, blanked):
            raise UnsupportedKernel(
                f"{src_dir.name}/: a helper reference survived flattening"
            )
    return text
