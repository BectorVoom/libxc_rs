//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1205/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1205<F: Float>(t115925: F, t25971: F, t24987: F, t8644: F, t101138: F, t26161: F, t31775: F, t1441: F, t6534: F, t2040: F, t33211: F, t7050: F, t119824: F, t119826: F, t119830: F, t120664: F, t22461: F, t24999: F, t26103: F, t26559: F, t27180: F, t27219: F, t6517: F, t7061: F, t7806: F) -> (F, F) {
    let t120899 = 3.0 * t115925 * t25971;
    let t120900 = t24987 * t8644;
    let t120907 = 2.0 * t26161 * t101138 * t31775;
    let t120908 = t1441 * t6534;
    let t120910 = 2.0 * t120908 * t2040;
    let t120912 = 2.0 * t33211 * t7050;
    let t120921 = 2.0 * t120664 * t26559 - 2.0 * t22461 * t7806 - 2.0 * t24999 * t7061 - 2.0 * t26103 * t7806 - 2.0 * t27180 * t6517 - 2.0 * t27219 * t6517 - t119824 - t119826 - t119830 - t120899 - t120900 + t120907 - t120910 - t120912;
    (t120908, t120921)
}
