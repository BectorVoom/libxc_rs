//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 992/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk992<F: Float>(t12240: F, t6977: F, t3851: F, t3901: F, t1337: F, t562: F, t3792: F, t550: F, t12177: F, t3897: F, t1338: F, t3879: F, t1352: F, t3773: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12241 = t6977 * t12240;
    let t12244 = t3901 * t3851;
    let t12247 = t1337 * t1337;
    let t12248 = 1.0 / t12247;
    let t12249 = t12248 * t562;
    let t12250 = t3792 * t550;
    let t12251 = t12177 * t12250;
    let t12252 = t12249 * t12251;
    let t12255 = t12177 * t3792;
    let t12256 = t3897 * t12255;
    let t12259 = t1338 * t3879;
    let t12260 = t12259 * t1352;
    let t12267 = t3773 * t68;
    (t12241, t12244, t12247, t12248, t12249, t12250, t12251, t12252, t12255, t12256, t12259, t12260, t12267)
}
