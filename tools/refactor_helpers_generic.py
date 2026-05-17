#!/usr/bin/env python3
"""
Batch refactor math/src/ helpers from concrete f64 to generic <F: Float>.

Usage:
  python3 tools/refactor_helpers_generic.py crates/kernels/math/src/helper.rs

Transformations:
1. Function signatures: f64 -> <F: Float>, params/returns f64 -> F
2. Literals: 1.0, 0.0, -1.0 -> F::new(1.0), F::new(0.0), F::new(-1.0)
3. Method calls: f64::method(x) -> F::method(x)
4. Named constants: const X: f64 = ... -> const X: f64 = ... (keep as const, wrap at use)
5. Comments: update doc comments to mention generic F
"""

import sys
import re

def refactor_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    original_content = content

    # Step 1: Update doc comments to mention generic F
    content = re.sub(
        r'(//! .*?f64)',
        r'\1\n//! Generic over `<F: Float>` to support both f64 and f32.',
        content,
        count=1
    )

    # Step 2: Convert function signatures
    # Pattern: pub fn name(args: f64, ...) -> f64 to pub fn name<F: Float>(args: F, ...) -> F
    def convert_signature(match):
        fn_def = match.group(0)
        # Check if already has <F: Float>
        if '<F: Float>' in fn_def:
            return fn_def

        # Add <F: Float> after function name
        fn_def = re.sub(
            r'(pub\s+(?:unsafe\s+)?fn\s+\w+)\(',
            r'\1<F: Float>(',
            fn_def
        )

        # Replace f64 params with F (but not in type hints inside arrays/references)
        fn_def = re.sub(
            r'\bf64\b(?!\s*>)',  # f64 not followed by >
            'F',
            fn_def
        )

        return fn_def

    # Match function signatures (pub fn to { or to :)
    content = re.sub(
        r'(#\[cube\]\s+pub\s+(?:unsafe\s+)?fn\s+\w+\s*(?:<F:\s*Float>)?\s*\([^{]*?\)\s*->\s*[^{;]*)',
        convert_signature,
        content,
        flags=re.MULTILINE
    )

    # Step 3: Replace f64 literals with F::new(...)
    # Pattern: 1.0, 0.0, -1.5, 1e-12, etc.
    def wrap_literal(match):
        lit = match.group(0)
        # Skip if already wrapped
        if 'F::new' in lit:
            return lit
        # Skip if it's a constant name or part of a larger expression
        return f'F::new({lit})'

    # Match numeric literals (but be careful not to double-wrap)
    # Pattern: space or operator followed by a float literal
    content = re.sub(
        r'(?<=[^\w>])((?:\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?(?:f64)?)\b(?!::new)',
        lambda m: f'F::new({m.group(1).rstrip("f64")})',
        content
    )

    # Also match negative literals: -1.0, -2.5e-3, etc.
    content = re.sub(
        r'(?<=[^\w>])(-(?:\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?(?:f64)?)\b(?!::new)',
        lambda m: f'F::new({m.group(1).rstrip("f64")})',
        content
    )

    # Step 4: Replace f64::method(x) with F::method(x)
    content = re.sub(
        r'\bf64::(\w+)',
        r'F::\1',
        content
    )

    # Step 5: Replace select(cond, lit, lit) patterns
    # These often have f64 suffix which needs wrapping
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

    # Step 6: Handle if/else with f64 literals
    content = re.sub(
        r'\b(result|[a-z_]\w*)\s*=\s*(\d+\.?\d*f64)',
        lambda m: f'{m.group(1)} = F::new({m.group(2).rstrip("f64")})',
        content
    )

    # Step 7: Handle named constant references - wrap at use
    # Pattern: PP0 + x * (PP1 + ...) -> F::new(PP0) + x * (F::new(PP1) + ...)
    # (This is complex, so we do a simple regex for common patterns)
    content = re.sub(
        r'\b([A-Z][A-Z0-9_]*)\b(?!\s*:)',  # Named constant (uppercase)
        lambda m: f'F::new({m.group(1)})' if m.group(1) not in ['TRUE', 'FALSE'] else m.group(1),
        content
    )

    # Step 8: Clean up double-wrapped literals (F::new(F::new(...)))
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

    # Step 10: Handle private helper functions that also need <F: Float>
    content = re.sub(
        r'(^|\n)(fn\s+\w+)\(',
        lambda m: m.group(1) + re.sub(
            r'(fn\s+\w+)\(',
            r'\1<F: Float>(',
            m.group(2) + '('
        ).rstrip('('),
        content,
        flags=re.MULTILINE
    )

    # Convert params in private fns
    content = re.sub(
        r'(fn\s+\w+<F:\s*Float>\s*\([^{]*?:\s*)f64(\b)',
        r'\1F\2',
        content
    )

    return content

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 refactor_helpers_generic.py <file.rs> [file2.rs ...]")
        sys.exit(1)

    for filepath in sys.argv[1:]:
        print(f"Refactoring {filepath}...")
        try:
            refactored = refactor_file(filepath)
            with open(filepath, 'w') as f:
                f.write(refactored)
            print(f"  ✓ Done: {filepath}")
        except Exception as e:
            print(f"  ✗ Error: {filepath}")
            print(f"    {e}")

if __name__ == '__main__':
    main()
