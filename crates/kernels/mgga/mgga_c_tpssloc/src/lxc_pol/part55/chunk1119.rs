//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1119/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1119<F: Float>(t27888: F, t7461: F, t25980: F, t7266: F, t31832: F, t7688: F, t25010: F, t8690: F, t116135: F, t25971: F, t26504: F, t2165: F, t26135: F, t652: F, t7423: F, t24969: F, t7467: F) -> (F, F, F, F, F, F, F, F, F) {
    let t123215 = t27888 * t7461;
    let t123217 = t7266 * t25980;
    let t123220 = t31832 * t7688;
    let t123228 = t8690 * t25010;
    let t123229 = t116135 * t25971;
    let t123235 = t8690 * t26504;
    let t123244 = t652 * t2165 * t26135;
    let t123272 = t7423 * t26135;
    let t123274 = t24969 * t7467;
    (t123215, t123217, t123220, t123228, t123229, t123235, t123244, t123272, t123274)
}
