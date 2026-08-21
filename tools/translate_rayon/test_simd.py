#!/usr/bin/env python3
"""Unit tests for tools/translate_rayon/simd.py translation in exact and fast math modes."""
import unittest
import simd


class TestSimdTranslation(unittest.TestCase):
    def test_exact_mode_unary_and_free(self):
        # f64::exp and f64::ln map to simd:: in exact mode
        self.assertEqual(simd.rewrite_calls("f64::exp(x)", math_mode="exact"), "(simd::exp(x))")
        self.assertEqual(simd.rewrite_calls("f64::ln(x)", math_mode="exact"), "(simd::ln(x))")
        # f64::atan and f64::tanh map to wide methods in exact mode
        self.assertEqual(simd.rewrite_calls("f64::atan(x)", math_mode="exact"), "((x).atan())")
        self.assertEqual(simd.rewrite_calls("f64::tanh(x)", math_mode="exact"), "((x).tanh())")
        self.assertEqual(simd.rewrite_calls("f64::sqrt(x)", math_mode="exact"), "((x).sqrt())")

    def test_fast_mode_unary_and_free(self):
        # In fast mode, all transcendentals map to rmath_fast free functions
        self.assertEqual(simd.rewrite_calls("f64::exp(x)", math_mode="fast"), "(rmath_fast::exp(x))")
        self.assertEqual(simd.rewrite_calls("f64::ln(x)", math_mode="fast"), "(rmath_fast::ln(x))")
        self.assertEqual(simd.rewrite_calls("f64::exp_m1(x)", math_mode="fast"), "(rmath_fast::expm1(x))")
        self.assertEqual(simd.rewrite_calls("f64::ln_1p(x)", math_mode="fast"), "(rmath_fast::log1p(x))")
        self.assertEqual(simd.rewrite_calls("f64::atan(x)", math_mode="fast"), "(rmath_fast::atan(x))")
        self.assertEqual(simd.rewrite_calls("f64::tanh(x)", math_mode="fast"), "(rmath_fast::tanh(x))")
        self.assertEqual(simd.rewrite_calls("f64::sinh(x)", math_mode="fast"), "(rmath_fast::sinh(x))")
        self.assertEqual(simd.rewrite_calls("f64::cosh(x)", math_mode="fast"), "(rmath_fast::cosh(x))")
        self.assertEqual(simd.rewrite_calls("f64::asin(x)", math_mode="fast"), "(rmath_fast::asin(x))")
        self.assertEqual(simd.rewrite_calls("f64::acos(x)", math_mode="fast"), "(rmath_fast::acos(x))")
        self.assertEqual(simd.rewrite_calls("erf(x)", math_mode="fast"), "(rmath_fast::erf(x))")
        self.assertEqual(simd.rewrite_calls("erfc(x)", math_mode="fast"), "(rmath_fast::erfc(x))")
        self.assertEqual(simd.rewrite_calls("pow_1_3(x)", math_mode="fast"), "(rmath_fast::cbrt(x))")
        # sqrt and abs remain wide methods in fast mode
        self.assertEqual(simd.rewrite_calls("f64::sqrt(x)", math_mode="fast"), "((x).sqrt())")
        self.assertEqual(simd.rewrite_calls("f64::abs(x)", math_mode="fast"), "((x).abs())")

    def test_binary_functions(self):
        # powf and atan2 in exact vs fast mode
        self.assertEqual(simd.rewrite_calls("f64::powf(x, y)", math_mode="exact"), "((x).powf_simd(y))")
        self.assertEqual(simd.rewrite_calls("f64::atan2(y, x)", math_mode="exact"), "((y).atan2(x))")
        self.assertEqual(simd.rewrite_calls("f64::powf(x, y)", math_mode="fast"), "(rmath_fast::pow(x, y))")
        self.assertEqual(simd.rewrite_calls("f64::atan2(y, x)", math_mode="fast"), "(rmath_fast::atan2(y, x))")

    def test_powers_expansion(self):
        self.assertEqual(simd.rewrite_calls("pow_2(x)"), "((x) * (x))")
        self.assertEqual(simd.rewrite_calls("pow_3(x)"), "((x) * (x) * (x))")
        self.assertEqual(simd.rewrite_calls("pow_1_4(x)"), "((x).sqrt().sqrt())")
        self.assertEqual(simd.rewrite_calls("pow_3_2(x)"), "((x) * (x).sqrt())")

    def test_piecewise_and_heaviside(self):
        res = simd.rewrite_calls("piecewise3(x <= 1.0, a, b)", math_mode="fast")
        self.assertIn(".select(", res)
        self.assertIn("simd_le", res)

        res5 = simd.rewrite_calls("piecewise5(x <= 1.0, a, x <= 2.0, b, c)", math_mode="fast")
        self.assertIn(".select(", res5)

        h_res = simd.rewrite_calls("Heaviside(x)", math_mode="fast")
        self.assertIn(".simd_ge(V_ZERO).select(V_ONE, V_ZERO)", h_res)

    def test_nested_expressions(self):
        nested = "f64::exp(f64::ln(x) + f64::powf(y, 2.0))"
        exact_res = simd.rewrite_calls(nested, math_mode="exact")
        self.assertEqual(exact_res, "(simd::exp((simd::ln(x)) + ((y).powf_simd(2.0))))")

        fast_res = simd.rewrite_calls(nested, math_mode="fast")
        self.assertEqual(fast_res, "(rmath_fast::exp((rmath_fast::ln(x)) + (rmath_fast::pow(y, 2.0))))")

    def test_simd_body_generation(self):
        lines = [
            "let t0 = rho[ip] * 2.0;",
            "let t1 = f64::exp(t0);",
            "zk[ip] += t1;",
        ]
        body_exact = simd.simd_body(lines, ["rho"], ["zk"], ["p0"], "my_func", math_mode="exact")
        self.assertIn("use libxc_rkernel_math::simd;", body_exact)
        self.assertNotIn("use libxc_rkernel_math::rmath_fast;", body_exact)
        self.assertIn("simd::exp", body_exact)

        body_fast = simd.simd_body(lines, ["rho"], ["zk"], ["p0"], "my_func", math_mode="fast")
        self.assertIn("use libxc_rkernel_math::rmath_fast;", body_fast)
        self.assertIn("rmath_fast::exp", body_fast)


if __name__ == "__main__":
    unittest.main()
