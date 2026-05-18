//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1078/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1078<F: Float>(t2122: F, t32514: F, t1186: F, t2144: F, t7284: F, t7287: F, t1090: F, t24601: F, t7301: F, t7391: F, t7300: F, t1251: F, t8887: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32515 = t2122 * t32514;
    let t32516 = t1186 * t32515;
    let t32519 = t7284 * t2144;
    let t32520 = t32519 * t7287;
    let t32523 = t32514 * t1090;
    let t32524 = t24601 * t32523;
    let t32529 = t7301 * t7391;
    let t32530 = t7300 * t32529;
    let t32537 = t8887 * t1251;
    (t32515, t32516, t32519, t32520, t32523, t32524, t32529, t32530, t32537)
}
