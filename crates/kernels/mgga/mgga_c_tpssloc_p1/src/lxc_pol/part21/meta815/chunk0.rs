//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2871/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2871<F: Float>(t10817: F, t17510: F, t17513: F, t42143: F, t17517: F, t10771: F, t10811: F, t10828: F, t14271: F, t14328: F, t14337: F, t14439: F, t14443: F, t14463: F, t1569: F, t2861: F, t2862: F, t2880: F, t2886: F, t2906: F, t2930: F, t49285: F, t5743: F, t5759: F, t5762: F, t5775: F, t5791: F, t60006: F, t60008: F, t60010: F, t60016: F, t60021: F, t60023: F) -> (F, F, F, F) {
    let t60025 = F::new(8.0) * t10817 * t17510;
    let t60027 = F::cast_from(0.1929837539843104208e3_f64) * t42143 * t17513;
    let t60029 = F::new(4.0) * t10817 * t17517;
    let t60030 = F::cast_from(0.64327917994770140268e2_f64) * t14271 * t14439 + F::cast_from(0.4138081033541872024e4_f64) * t49285 * t14443 + F::new(6.0) * t2886 * t5743 * t2880 + F::cast_from(0.11579025239058625248e4_f64) * t10811 * t5762 * t2862 - F::new(4.0) * t2861 * t1569 * t14328 + F::cast_from(0.70178683471615754484e1_f64) * t14337 * t14463 + F::new(6.0) * t2886 * t5759 * t2862 - F::cast_from(0.14035736694323150897e2_f64) * t10828 * t5775 * t2906 + t60006 - t60008 + t60010 - F::new(24.0) * t10771 * t5743 * t2862 + t60016 + F::cast_from(0.35089341735807877242e1_f64) * t2930 * t5791 * t2906 - t60021 - t60023 + t60025 + t60027 + t60029;
    (t60025, t60027, t60029, t60030)
}
