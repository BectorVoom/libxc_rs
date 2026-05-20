//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2190/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2190<F: Float>(t16398: F, t19890: F, t12283: F, t19972: F, t16046: F, t1814: F, t12250: F, t5286: F, t1372: F, t6414: F, t1338: F, t20009: F) -> (F, F, F, F, F, F) {
    let t57450 = t16398 * t19890;
    let t57457 = t12283 * t19972;
    let t57530 = t1814 * t16046;
    let t57568 = t12250 * t5286;
    let t57618 = t1372 * t6414;
    let t57659 = t1338 * t20009;
    (t57450, t57457, t57530, t57568, t57618, t57659)
}
