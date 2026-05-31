//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 869/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk869<F: Float>(t271: F, t2775: F, t974: F, t2769: F, t632: F, t344: F, t9288: F, t698: F, t976: F, t979: F, t973: F, t2970: F, t2999: F) -> (F, F, F, F, F) {
    let t10213 = F::cast_from(1.0_f64) / t271 / t2775;
    let t10214 = t974 * t10213;
    let t10216 = F::cast_from(1.0_f64) / t2769 / t632;
    let t10217 = t344 * t10216;
    let t10218 = t10217 * t9288;
    let t10219 = t10214 * t10218;
    let t10224 = t698 * t976;
    let t10225 = t10224 * t979;
    let t10226 = t973 * t10225;
    let t10228 = t2970 * t2999;
    (t10213, t10216, t10219, t10226, t10228)
}
