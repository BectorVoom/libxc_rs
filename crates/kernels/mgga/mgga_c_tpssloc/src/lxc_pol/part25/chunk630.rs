//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 630/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk630<F: Float>(t2932: F, t950: F, t2978: F, t60: F, t344: F, t2987: F, t340: F, t974: F, t247: F, t375: F) -> (F, F, F, F, F, F) {
    let t4497 = t2932 * t950;
    let t4509 = t60 * t2978;
    let t4510 = t4509 * t344;
    let t4518 = t2987 * t344;
    let t4546 = t974 * t340;
    let t4582 = t247 * t375;
    (t4497, t4509, t4510, t4518, t4546, t4582)
}
