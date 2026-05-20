//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2240/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2240<F: Float>(t40: F, t4199: F, t9713: F, t41255: F, t41259: F, t41265: F, t1471: F, t31: F, t9898: F, t10913: F, t12606: F, t12950: F, t1430: F, t2244: F, t2250: F, t4007: F, t4010: F, t4104: F, t45872: F, t607: F, t75: F, t767: F, t9258: F, t9288: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t46376 = t4199 * t9713;
    let t46377 = F::cast_from(0.5848223622634646207e0_f64) * t46376;
    let t46384 = F::cast_from(0.17544670867903938621e1_f64) * t41255;
    let t46385 = F::cast_from(0.17544670867903938621e1_f64) * t41259;
    let t46386 = F::cast_from(0.5848223622634646207e0_f64) * t41265;
    let t46387 = t31 * t1471;
    let t46389 = F::new(24.0) * t46387 * t9898;
    let t46407 = piecewise3::<F>(t146, F::new(0.0), -F::new(56.0) / F::new(81.0) * t4007 * t9288 + F::new(8.0) / F::new(9.0) * t4010 * t2244 + F::new(8.0) / F::new(9.0) * t1430 * t10913 - F::new(2.0) / F::new(3.0) * t75 * t12606 * t607 - F::new(2.0) / F::new(3.0) * t12950 * t2250 - F::new(2.0) / F::new(9.0) * t4104 * t9258 + F::new(2.0) / F::new(3.0) * t767 * t45872);
    (t46377, t46384, t46385, t46386, t46389, t46407)
}
