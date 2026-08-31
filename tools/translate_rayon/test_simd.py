#!/usr/bin/env python3
"""Unit tests for tools/translate_rayon/simd.py translation in exact and fast math modes."""
import unittest
import simd


class TestSimdTranslation(unittest.TestCase):
    def test_exact_mode_unary_and_free(self):
        # All transcendentals map to simd:: in exact mode
        self.assertEqual(simd.rewrite_calls("f64::exp(x)"), "(simd::exp(x))")
        self.assertEqual(simd.rewrite_calls("f64::ln(x)"), "(simd::ln(x))")
        self.assertEqual(simd.rewrite_calls("f64::exp_m1(x)"), "(simd::expm1(x))")
        self.assertEqual(simd.rewrite_calls("f64::ln_1p(x)"), "(simd::log1p(x))")
        self.assertEqual(simd.rewrite_calls("f64::atan(x)"), "(simd::atan(x))")
        self.assertEqual(simd.rewrite_calls("f64::tanh(x)"), "(simd::tanh(x))")
        self.assertEqual(simd.rewrite_calls("f64::sinh(x)"), "(simd::sinh(x))")
        self.assertEqual(simd.rewrite_calls("f64::cosh(x)"), "(simd::cosh(x))")
        self.assertEqual(simd.rewrite_calls("f64::asin(x)"), "(simd::asin(x))")
        self.assertEqual(simd.rewrite_calls("f64::acos(x)"), "(simd::acos(x))")
        self.assertEqual(simd.rewrite_calls("f64::atanh(x)"), "(simd::atanh(x))")
        self.assertEqual(simd.rewrite_calls("f64::sin(x)"), "(simd::sin(x))")
        self.assertEqual(simd.rewrite_calls("f64::cos(x)"), "(simd::cos(x))")
        self.assertEqual(simd.rewrite_calls("f64::tan(x)"), "(simd::tan(x))")
        self.assertEqual(simd.rewrite_calls("erf(x)"), "(simd::erf(x))")
        self.assertEqual(simd.rewrite_calls("erfc(x)"), "(simd::erfc(x))")
        self.assertEqual(simd.rewrite_calls("pow_1_3(x)"), "(simd::cbrt(x))")
        self.assertEqual(simd.rewrite_calls("pow_2_3(x)"), "(simd::pow_2_3(x))")
        self.assertEqual(simd.rewrite_calls("pow_4_3(x)"), "(simd::pow_4_3(x))")
        self.assertEqual(simd.rewrite_calls("pow_5_3(x)"), "(simd::pow_5_3(x))")
        self.assertEqual(simd.rewrite_calls("pow_7_3(x)"), "(simd::pow_7_3(x))")
        # rmath:: prefixes
        self.assertEqual(simd.rewrite_calls("rmath::exp(x)"), "(simd::exp(x))")
        self.assertEqual(simd.rewrite_calls("rmath::ln(x)"), "(simd::ln(x))")
        self.assertEqual(simd.rewrite_calls("rmath::atan(x)"), "(simd::atan(x))")
        self.assertEqual(simd.rewrite_calls("rmath::tanh(x)"), "(simd::tanh(x))")
        self.assertEqual(simd.rewrite_calls("rmath::erf(x)"), "(simd::erf(x))")
        self.assertEqual(simd.rewrite_calls("rmath::erfc(x)"), "(simd::erfc(x))")

        # sqrt and abs remain native wide methods in exact mode
        self.assertEqual(simd.rewrite_calls("f64::sqrt(x)"), "((x).sqrt())")
        self.assertEqual(simd.rewrite_calls("f64::abs(x)"), "((x).abs())")
        self.assertEqual(simd.rewrite_calls("rmath::sqrt(x)"), "((x).sqrt())")
        self.assertEqual(simd.rewrite_calls("rmath::abs(x)"), "((x).abs())")

        # binary calls in exact mode
        self.assertEqual(simd.rewrite_calls("f64::powf(x, y)"), "(simd::pow(x, y))")
        self.assertEqual(simd.rewrite_calls("rmath::pow(x, y)"), "(simd::pow(x, y))")
        self.assertEqual(simd.rewrite_calls("f64::atan2(y, x)"), "(simd::atan2(y, x))")
        self.assertEqual(simd.rewrite_calls("rmath::atan2(y, x)"), "(simd::atan2(y, x))")

    def test_powers_expansion(self):
        self.assertEqual(simd.rewrite_calls("pow_2(x)"), "((x) * (x))")
        self.assertEqual(simd.rewrite_calls("pow_3(x)"), "((x) * (x) * (x))")
        self.assertEqual(simd.rewrite_calls("pow_1_4(x)"), "((x).sqrt().sqrt())")
        self.assertEqual(simd.rewrite_calls("pow_3_2(x)"), "((x) * (x).sqrt())")

    def test_piecewise_and_heaviside(self):
        res = simd.rewrite_calls("piecewise3(x <= 1.0, a, b)")
        self.assertIn(".select(", res)
        self.assertIn("simd_le", res)

        res5 = simd.rewrite_calls("piecewise5(x <= 1.0, a, x <= 2.0, b, c)")
        self.assertIn(".select(", res5)

        h_res = simd.rewrite_calls("Heaviside(x)")
        self.assertIn(".simd_ge(V_ZERO).select(V_ONE, V_ZERO)", h_res)

    def test_nested_expressions(self):
        nested = "f64::exp(f64::ln(x) + f64::powf(y, 2.0))"
        exact_res = simd.rewrite_calls(nested)
        self.assertEqual(exact_res, "(simd::exp((simd::ln(x)) + (simd::pow(y, 2.0))))")


    def test_simd_body_generation(self):
        lines = [
            "let t0 = rho[ip] * 2.0;",
            "let t1 = f64::exp(t0);",
            "zk[ip] += t1;",
        ]
        body_exact = simd.simd_body(lines, ["rho"], ["zk"], ["p0"], "my_func")
        self.assertIn("use libxc_rkernel_math::simd;", body_exact)
        self.assertNotIn("use libxc_rkernel_math::rmath_fast;", body_exact)
        self.assertIn("simd::exp", body_exact)

        # There is no approximate mode: the emitter has one math path, and it
        # is the bit-exact one. A kernel must never reach rmath's Fast free
        # functions, which is what `rmath_fast` used to expose.
        self.assertNotIn("rmath_fast", body_exact)


if __name__ == "__main__":
    unittest.main()

