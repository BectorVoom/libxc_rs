//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 909/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk909<F: Float>(t14327: F, t558: F, t3851: F, t75201: F, t7782: F, t75087: F, t7835: F, t74812: F, t74816: F, t11723: F, t69507: F, t12012: F, t69511: F) -> (F, F, F, F, F, F, F, F) {
    let t76270 = t14327 * t558;
    let t76271 = t3851 * t76270;
    let t76273 = t7782 * t75201;
    let t76275 = t7835 * t75087;
    let t76277 = t7835 * t74812;
    let t76279 = t7835 * t74816;
    let t76281 = t69507 * t11723;
    let t76283 = t69511 * t12012;
    (t76270, t76271, t76273, t76275, t76277, t76279, t76281, t76283)
}
