//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1693/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1693<F: Float>(t3789: F, t5234: F, t3798: F, t1354: F, t12211: F, t5223: F, t1307: F, t210: F, t5226: F, t1810: F, t3719: F, t3804: F, t820: F) -> (F, F, F, F, F, F) {
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    let t16290 = F::new(7.0) / F::new(2304.0) * t16288 * t1354;
    let t16294 = F::new(7.0) / F::new(24.0) * t12211 * t5223;
    let t16296 = t210 * t5226 * t1307;
    let t16300 = t210 * t1810 * t3719;
    let t16305 = t3804 * t820;
    (t16285, t16290, t16294, t16296, t16300, t16305)
}
