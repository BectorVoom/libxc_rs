//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2239/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2239<F: Float>(t4541: F, t984: F, t23384: F, t25467: F, t25459: F, t1058: F, t1060: F, t11037: F, t13933: F, t14526: F, t1615: F, t1920: F, t1948: F, t1949: F, t23346: F, t23571: F, t23670: F, t25541: F, t25558: F, t25713: F, t25718: F, t3076: F, t3186: F, t3188: F, t345: F, t6687: F, t7622: F, t88941: F, t89312: F) -> (F, F) {
    let t89349 = t4541 * t984;
    let t89360 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25467;
    let t89362 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25459;
    let t89363 = F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t345 * t1948 * t14526 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t25718 + t3076 * t7622 - F::new(2.0) * t11037 * t25558 + F::new(2.0) * t3186 * t89312 * t3188 + t1058 * t23571 * t1615 * t1060 - F::cast_from(0.43864908449286038306e-1_f64) * t23670 * t25541 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t89349 * t25713 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t88941 * t25713 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t13933 * t1949 - t89360 - t89362;
    (t89349, t89363)
}
