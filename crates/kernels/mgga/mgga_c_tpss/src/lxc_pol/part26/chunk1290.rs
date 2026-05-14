//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1290/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1290<F: Float>(t3431: F, t77: F, t84: F, t1976: F, t4580: F, t13330: F, t578: F, t13336: F, t38: F, t42690: F, t13442: F, t76: F, t4622: F, t619: F, t1760: F, t21017: F, t61845: F) -> (F, F, F, F, F, F, F, F) {
    let t69242 = t77 * t84 * t3431;
    let t69245 = t1976 * t4580;
    let t69248 = t578 * t13330;
    let t69251 = t578 * t13336;
    let t69281 = t42690 * t38;
    let t69338 = t76 * t13442;
    let t69355 = t77 * t4622 * t619;
    let t69372 = 6.0 * t1760 * t61845 * t21017;
    (t69242, t69245, t69248, t69251, t69281, t69338, t69355, t69372)
}
