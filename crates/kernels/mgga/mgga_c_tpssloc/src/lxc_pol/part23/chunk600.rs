//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 600/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk600<F: Float>(t457: F, t974: F, t1721: F, t225: F, t1222: F, t1731: F, t1744: F, t1229: F, t3247: F, t3242: F, t3584: F, t1653: F, t248: F, t3521: F) -> (F, F, F, F, F, F, F) {
    let t4934 = t974 * t457;
    let t4945 = t1721 * t225;
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4972 = t1229 * t3247;
    let t4987 = t3584 * t3242;
    let t4993 = t248 * t3521 * t1653;
    (t4934, t4945, t4957, t4959, t4972, t4987, t4993)
}
