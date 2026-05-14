//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 776/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk776<F: Float>(t75087: F, t7835: F, t74812: F, t74816: F, t11723: F, t69507: F, t12012: F, t69511: F, t11729: F, t11732: F, t69433: F, t12140: F, t69176: F, t305: F, t76062: F, t75674: F, t793: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76275 = t7835 * t75087;
    let t76277 = t7835 * t74812;
    let t76279 = t7835 * t74816;
    let t76281 = t69507 * t11723;
    let t76283 = t69511 * t12012;
    let t76285 = t69507 * t11729;
    let t76287 = t69433 * t11732;
    let t76289 = t69176 * t12140;
    let t76291 = t305 * t76062;
    let t76292 = 0.79828278012425390427e-1 * t76291;
    let t76305 = t793 * t75674;
    (t76275, t76277, t76279, t76281, t76283, t76285, t76287, t76289, t76292, t76305)
}
