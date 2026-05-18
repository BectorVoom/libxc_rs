#!/usr/bin/env python3
"""
Batch refactor math/src/ helpers from concrete f64 to generic <F: Float>.

D-20 cast_from policy classifier (added 2026-05-18, 4th-iter recovery):
Every `F::new(IDENT)` site is classified by symbol class and rewritten
accordingly. This replaces the prior naive blanket `F::new(IDENT)` wrap that
caused the 11-06 HALT (515 errors, dominated by 447 × E0308
`expected f32, found f64` from `F::new(<f64_const>)` against
`Float::new(val: f32)`).

Policy classes (per CONTEXT.md D-20):
- f64 const (SQRT_DBL_EPSILON, LOG_DBL_MAX, ...)   -> F::cast_from(IDENT)
- f32 const                                         -> F::new(IDENT) (keep)
- Doc-comment / string-literal context              -> bare IDENT (revert)
- Non-generic file (deferred.rs)                    -> bare IDENT (revert)
- Numeric literal with _f64 suffix                  -> F::new(N.N) repair
- Range operator (`0..500`)                         -> preserve
- Double-wrap (F::F::new(MAX))                      -> F::cast_from(f64::MAX)

Usage:
  python3 tools/refactor_helpers_generic.py [--dry-run] [--report-classifications] FILES...

Flags:
  --dry-run                  Report changes without writing files
  --report-classifications   Print per-class site counts
"""

import sys
import re
import argparse
from pathlib import Path

# ----------------------------------------------------------------------------
# D-20 cast_from classifier
# ----------------------------------------------------------------------------

# Match `const NAME: f64 = ...` and `pub const NAME: f64 = ...`
F64_CONST_PATTERN = re.compile(r'^\s*(?:pub\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*f64\s*=')
F32_CONST_PATTERN = re.compile(r'^\s*(?:pub\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*f32\s*=')

# Detect at least one generic-over-F function in the file
GENERIC_F_FN_PATTERN = re.compile(r'fn\s+\w+\s*<\s*F\s*:\s*Float\s*>')

# Match F::new(UPPERCASE_IDENT) — multi-char uppercase identifier
F_NEW_IDENT_PATTERN = re.compile(r'F::new\(([A-Z][A-Z0-9_]+)\)')


def classify_file(path: Path) -> dict:
    """Return per-file metadata used by the per-site classifier."""
    src = path.read_text()
    lines = src.split('\n')
    f64_consts = set()
    f32_consts = set()
    for line in lines:
        m = F64_CONST_PATTERN.match(line)
        if m:
            f64_consts.add(m.group(1))
        m = F32_CONST_PATTERN.match(line)
        if m:
            f32_consts.add(m.group(1))
    is_generic = bool(GENERIC_F_FN_PATTERN.search(src))
    return {
        'src': src,
        'f64_consts': f64_consts,
        'f32_consts': f32_consts,
        'is_generic': is_generic,
        'path': str(path),
    }


def classify_site(ident: str, meta: dict, line_context: str) -> tuple:
    """Classify a single F::new(IDENT) site. Returns (replacement, class_tag).

    Per D-20 policy:
    - f64 const  -> F::cast_from(IDENT)        (cast_from)
    - f32 const  -> F::new(IDENT) (keep)       (keep_f_new)
    - doc-comment / string-literal context     (revert -> bare IDENT)
    - non-generic file                          (revert -> bare IDENT)
    - unknown identifier (likely local var)     (revert -> bare IDENT, defensive)
    """
    # Doc-comment context (line starts with `///`, `//!`, or `//`)
    stripped = line_context.lstrip()
    if stripped.startswith('///') or stripped.startswith('//!') or stripped.startswith('//'):
        return (ident, 'revert')

    # String-literal context: odd number of `"` to the left means inside string
    # We approximate by counting all `"` on the line and checking parity
    # (correct for single-line literals; multi-line raw strings are rare in helpers)
    quote_count = line_context.count('"')
    if quote_count % 2 != 0:
        return (ident, 'revert')

    # Non-generic file (e.g., deferred.rs) — full revert
    if not meta['is_generic']:
        return (ident, 'revert')

    # f64 const → cast_from (architectural fix for the 11-06 HALT root cause)
    if ident in meta['f64_consts']:
        return (f"F::cast_from({ident})", 'cast_from')

    # f32 const → keep F::new
    if ident in meta['f32_consts']:
        return (f"F::new({ident})", 'keep_f_new')

    # Unknown identifier — likely a function param / local var / external const.
    # Defensive: revert to bare ident (avoids spurious wraps).
    return (ident, 'revert')


def apply_cast_from_policy(src: str, meta: dict, counters: dict) -> str:
    """Apply D-20 cast_from classifier to every F::new(IDENT) site (multi-char IDENT)."""
    out_lines = []
    for line in src.split('\n'):
        def repl(m):
            ident = m.group(1)
            new_text, class_tag = classify_site(ident, meta, line)
            counters[class_tag] = counters.get(class_tag, 0) + 1
            return new_text
        new_line = F_NEW_IDENT_PATTERN.sub(repl, line)
        out_lines.append(new_line)
    return '\n'.join(out_lines)


# ----------------------------------------------------------------------------
# D-23 surgical fix patterns (defensive — Task 1 applied these via Edit, but
# the script must self-validate and remain idempotent).
# ----------------------------------------------------------------------------

SURGICAL_PATTERNS = [
    # _f64 suffix mis-wraps (D-23 mbrxc.rs:145, 153)
    # F::new(3.)0_f64 -> F::new(3.0)
    (re.compile(r'F::new\((\d+)\.\)(\d+)_f64'), r'F::new(\1.\2)'),
    # Range-op mis-wrap (D-23 mbrxc.rs:154)
    # for _ in 0.F::new(.500) -> for _ in 0..500
    (re.compile(r'for _ in 0\.F::new\(\.(\d+)\)'), r'for _ in 0..\1'),
    # Double-wrap (D-23 special.rs:224) -> F::cast_from(f64::MAX)
    (re.compile(r'F::F::new\(MAX\)'), r'F::cast_from(f64::MAX)'),
]


def apply_surgical_fixes(src: str, counters: dict) -> str:
    for pat, repl in SURGICAL_PATTERNS:
        new_src, n = pat.subn(repl, src)
        if n:
            counters['surgical'] = counters.get('surgical', 0) + n
        src = new_src
    return src


# ----------------------------------------------------------------------------
# Legacy bulk transformer (pre-D-20). KEEP as fallback for files that still
# need full f64->F migration. The cast_from policy runs FIRST and handles
# F::new(IDENT) sites; the legacy passes handle signatures, numeric literals,
# method calls, etc.
# ----------------------------------------------------------------------------

def legacy_refactor(content: str) -> str:
    # Step 1: Update doc comments to mention generic F
    content = re.sub(
        r'(//! .*?f64)',
        r'\1\n//! Generic over `<F: Float>` to support both f64 and f32.',
        content,
        count=1
    )

    # Step 2: Convert function signatures
    def convert_signature(match):
        fn_def = match.group(0)
        if '<F: Float>' in fn_def:
            return fn_def
        fn_def = re.sub(
            r'(pub\s+(?:unsafe\s+)?fn\s+\w+)\(',
            r'\1<F: Float>(',
            fn_def
        )
        fn_def = re.sub(
            r'\bf64\b(?!\s*>)',
            'F',
            fn_def
        )
        return fn_def

    content = re.sub(
        r'(#\[cube\]\s+pub\s+(?:unsafe\s+)?fn\s+\w+\s*(?:<F:\s*Float>)?\s*\([^{]*?\)\s*->\s*[^{;]*)',
        convert_signature,
        content,
        flags=re.MULTILINE
    )

    # Step 3: Replace f64 literals with F::new(...)
    content = re.sub(
        r'(?<=[^\w>])((?:\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?(?:f64)?)\b(?!::new)',
        lambda m: f'F::new({m.group(1).rstrip("f64")})',
        content
    )
    content = re.sub(
        r'(?<=[^\w>])(-(?:\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?(?:f64)?)\b(?!::new)',
        lambda m: f'F::new({m.group(1).rstrip("f64")})',
        content
    )

    # Step 4: f64::method(x) -> F::method(x)
    content = re.sub(
        r'\bf64::(\w+)',
        r'F::\1',
        content
    )

    # Step 5: select(cond, lit, lit) suffix fix
    content = re.sub(
        r'select\(([^,]+),\s*(\d+\.?\d*f64),\s*([^)]+)\)',
        lambda m: f'select({m.group(1)}, F::new({m.group(2).rstrip("f64")}), {m.group(3)})',
        content
    )
    content = re.sub(
        r'select\(([^,]+),\s*([^,]+),\s*(\d+\.?\d*f64)\)',
        lambda m: f'select({m.group(1)}, {m.group(2)}, F::new({m.group(3).rstrip("f64")}))',
        content
    )

    # Step 6: if/else with f64 literals on RHS of assignment
    content = re.sub(
        r'\b(result|[a-z_]\w*)\s*=\s*(\d+\.?\d*f64)',
        lambda m: f'{m.group(1)} = F::new({m.group(2).rstrip("f64")})',
        content
    )

    # Step 7 (legacy): named-constant blanket wrap — DISABLED by D-20.
    # The cast_from classifier (applied BEFORE this legacy pass) already handles
    # F::new(IDENT) classification. Leaving the legacy Step 7 enabled would
    # re-wrap identifiers, producing F::new(F::cast_from(X)) and undoing the fix.
    # Keep this comment as a permanent reminder: do NOT re-enable Step 7.

    # Step 8: Clean up double-wrapped literals
    content = re.sub(
        r'F::new\(F::new\(([^)]+)\)\)',
        r'F::new(\1)',
        content
    )

    # Step 9: Fix const declarations (should stay as f64)
    content = re.sub(
        r'\bconst\s+(\w+):\s+F\s+=',
        r'const \1: f64 =',
        content
    )

    return content


# ----------------------------------------------------------------------------
# Top-level per-file driver
# ----------------------------------------------------------------------------

def transform_file(path: Path, dry_run: bool, counters: dict, run_legacy: bool = False) -> bool:
    """Apply D-20 cast_from policy + D-23 surgical fixes to the file.

    Returns True if the file content changed.

    By default (run_legacy=False) we ONLY apply the cast_from classifier and
    surgical fixes. The legacy bulk refactor is opt-in via run_legacy=True
    (used when a file is being migrated f64 -> F for the first time).
    """
    meta = classify_file(path)
    new_src = meta['src']

    # D-20 cast_from classifier (per-site classification of F::new(IDENT))
    new_src = apply_cast_from_policy(new_src, meta, counters)

    # D-23 surgical defensive patterns
    new_src = apply_surgical_fixes(new_src, counters)

    # Optional legacy bulk pass (off by default; for new-file migrations)
    if run_legacy:
        new_src = legacy_refactor(new_src)

    changed = new_src != meta['src']
    if changed and not dry_run:
        path.write_text(new_src)
    return changed


def main():
    parser = argparse.ArgumentParser(
        prog='refactor_helpers_generic.py',
        description='Apply D-20 cast_from policy + D-23 surgical fixes to math/src helpers.',
    )
    parser.add_argument('files', nargs='+', help='Files to transform (no globbing)')
    parser.add_argument('--dry-run', action='store_true',
                        help='Report changes without writing files')
    parser.add_argument('--report-classifications', action='store_true',
                        help='Print per-class site counts after run')
    parser.add_argument('--run-legacy', action='store_true',
                        help='Also run the legacy f64->F bulk refactor (off by default)')
    args = parser.parse_args()

    total_counters = {}
    changed_files = []

    for fp in args.files:
        path = Path(fp)
        if not path.is_file():
            print(f"SKIP (not a file): {fp}", file=sys.stderr)
            continue
        file_counters = {}
        try:
            changed = transform_file(path, args.dry_run, file_counters, run_legacy=args.run_legacy)
        except Exception as e:
            print(f"ERROR on {fp}: {e}", file=sys.stderr)
            continue
        for k, v in file_counters.items():
            total_counters[k] = total_counters.get(k, 0) + v
        if changed:
            changed_files.append(fp)
            verb = 'DRY-RUN-WOULD-CHANGE' if args.dry_run else 'CHANGED'
            cast_from = file_counters.get('cast_from', 0)
            revert = file_counters.get('revert', 0)
            keep = file_counters.get('keep_f_new', 0)
            surgical = file_counters.get('surgical', 0)
            print(f"{verb}: {fp} | cast_from={cast_from} revert={revert} keep_f_new={keep} surgical={surgical}")
        else:
            print(f"UNCHANGED: {fp}")

    if args.report_classifications:
        print("---")
        print(f"Files changed: {len(changed_files)}")
        print(f"Total cast_from rewrites:  {total_counters.get('cast_from', 0)}")
        print(f"Total revert (bare ident): {total_counters.get('revert', 0)}")
        print(f"Total keep_f_new:          {total_counters.get('keep_f_new', 0)}")
        print(f"Total surgical fixes:      {total_counters.get('surgical', 0)}")


if __name__ == '__main__':
    main()
