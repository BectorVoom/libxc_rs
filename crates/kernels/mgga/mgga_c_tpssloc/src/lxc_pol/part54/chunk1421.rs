//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1421/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1421<F: Float>(t1985: F, t8621: F, t90739: F, t115545: F, t1992: F, t26355: F, t22633: F, t22635: F, t31549: F, t5187: F, t33272: F, t81228: F, t81326: F) -> (F, F, F, F) {
    let t122260 = t1985 * t90739 * t8621;
    let t122270 = t1992 * t115545 * t26355;
    let t122278 = t22633 * t22635 * t31549 * t5187;
    let t122281 = t81228 * t81326 * t33272;
    (t122260, t122270, t122278, t122281)
}
