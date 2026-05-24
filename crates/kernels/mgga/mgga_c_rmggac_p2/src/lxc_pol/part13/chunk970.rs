//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 970/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk970<F: Float>(t2118: F, t41032: F, t22: F, t2353: F, t26531: F, t5184: F, t649: F, t8746: F, t41209: F, t8750: F, t41212: F, t41215: F, t7603: F) -> (F, F, F, F, F, F, F) {
    let t41271 = t2118 * t41032;
    let t41274 = t26531 * t22 * t2353;
    let t41276 = t649 * t5184;
    let t41277 = t8746 * t41276;
    let t41279 = t8750 * t41209;
    let t41281 = t8750 * t41212;
    let t41283 = t7603 * t41215;
    (t41271, t41274, t41276, t41277, t41279, t41281, t41283)
}
