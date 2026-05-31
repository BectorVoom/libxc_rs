//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1150/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1150<F: Float>(t2095: F, t31035: F, t1983: F, t6999: F, t8640: F, t1307: F, t2018: F, t24432: F, t22574: F, t191: F, t192: F, t7166: F) -> (F, F, F, F, F, F, F, F) {
    let t31295 = t2095 * t31035;
    let t31296 = t1983 * t31295;
    let t31297 = t8640 * t6999;
    let t31298 = t1983 * t31297;
    let t31299 = t2018 * t1307;
    let t31300 = t24432 * t31299;
    let t31302 = F::cast_from(3.0_f64) * t22574 * t31300;
    let t31304 = t7166 * t191 * t192;
    (t31295, t31296, t31297, t31298, t31299, t31300, t31302, t31304)
}
