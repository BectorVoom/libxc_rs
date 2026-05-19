//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 910/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk910<F: Float>(t11729: F, t69507: F, t11732: F, t69433: F, t12140: F, t69176: F, t305: F, t76062: F, t75674: F, t793: F, t5259: F, t75515: F) -> (F, F, F, F, F, F) {
    let t76285 = t69507 * t11729;
    let t76287 = t69433 * t11732;
    let t76289 = t69176 * t12140;
    let t76291 = t305 * t76062;
    let t76292 = F::cast_from(0.79828278012425390427e-1_f64) * t76291;
    let t76305 = t793 * t75674;
    let t76310 = t5259 * t75515;
    (t76285, t76287, t76289, t76292, t76305, t76310)
}
