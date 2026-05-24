//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 487/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk487<F: Float>(t1624: F, t352: F, t1627: F, t326: F, t874: F) -> (F, F, F) {
    let t5260 = t1624 * t352;
    let t5263 = t1627 * t352;
    let t5266 = t326 * t874;
    (t5260, t5263, t5266)
}
