//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2161/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2161<F: Float>(t11545: F, t241: F, t3241: F, t11229: F, t699: F, t11232: F, t242: F, t281: F, t415: F, t2394: F, t3253: F) -> (F, F, F, F, F, F, F) {
    let t43761 = t241 * t11545;
    let t43762 = t3241 * t3241;
    let t43763 = F::new(1.0) / t43762;
    let t43768 = t699 * t11229;
    let t43770 = t699 * t11232;
    let t43776 = t281 * t242 * t415;
    let t43777 = F::cast_from(0.13490888888888888889e1_f64) * t43776;
    let t43780 = t2394 * t3253;
    (t43761, t43763, t43768, t43770, t43776, t43777, t43780)
}
