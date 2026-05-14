//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1015/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1015<F: Float>(t3120: F, t3131: F, t10482: F, t3040: F, t1043: F, t2770: F, t10277: F, t3061: F, t10216: F, t10969: F, t1022: F, t883: F, t607: F, t360: F, t10632: F, t2906: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13980 = t3131 * t3120;
    let t13985 = t10482 * t3040;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    let t14187 = t10969 * t10216;
    let t14212 = t1022 * t883;
    let t14213 = t14212 * t607;
    let t14219 = t360 * t883;
    let t14220 = t14219 * t607;
    let t14227 = t607 * t1022;
    let t14228 = t14227 * t360;
    let t14259 = t10632 * t2906;
    (t13980, t13985, t14164, t14172, t14187, t14213, t14220, t14228, t14259)
}
