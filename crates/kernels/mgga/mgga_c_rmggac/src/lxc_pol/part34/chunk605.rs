//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 605/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk605<F: Float>(t2046: F, t641: F, t7296: F, t14327: F, t333: F, t3928: F, t2048: F, t338: F, t352: F, t1550: F, t14089: F, t34683: F, t7557: F, t14078: F, t7494: F, t1330: F, t22: F, t262: F) -> (F, F, F, F, F, F, F, F, F) {
    let t68735 = t2046 * t7296 * t641;
    let t68737 = t14327 * t333;
    let t68738 = t3928 * t68737;
    let t68739 = 0.23948483403727617128e0 * t68738;
    let t68740 = t338 * t2048;
    let t68741 = t68740 * t352;
    let t68742 = t1550 * t68741;
    let t68751 = t14089 * t34683 * t7557;
    let t68753 = t7494 * t14078;
    let t68756 = t1330 * t22 * t262;
    (t68735, t68737, t68739, t68740, t68741, t68742, t68751, t68753, t68756)
}
