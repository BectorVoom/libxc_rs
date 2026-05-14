//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 548/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk548<F: Float>(t4522: F, t977: F, t135: F, t1599: F, t973: F, t1597: F, t2987: F, t2990: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F, t340: F, t343: F) -> (F, F, F, F) {
    let t4523 = t977 * t4522;
    let t4528 = t135 * t1599;
    let t4529 = t973 * t4528;
    let t4531 = t2987 * t1597;
    let t4532 = t4531 * t2990;
    let t4540 = -t3003 - t2824 / 9.0 - t4384 / 9.0 + t4387 / 18.0 - t4390 / 3.0 + t4393 / 6.0;
    let t4541 = t340 * t4540;
    let t4542 = t4541 * t343;
    (t4523, t4529, t4532, t4542)
}
