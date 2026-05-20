//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1296/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1296<F: Float>(t31550: F, t81228: F, t81326: F, t31551: F, t81159: F, t115352: F, t6897: F, t6907: F, t3886: F, t7213: F, t225: F, t31585: F) -> (F, F, F, F, F) {
    let t115586 = t81228 * t81326 * t31550;
    let t115596 = t81159 * t31551;
    let t115601 = t6897 * t115352 * t6907;
    let t115614 = t3886 * t7213;
    let t115619 = t31585 * t225;
    (t115586, t115596, t115601, t115614, t115619)
}
