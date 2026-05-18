//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1126/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1126<F: Float>(t1049: F, t225: F, t344: F, t10189: F, t1926: F, t221: F, t1921: F, t6733: F, t23383: F, t6712: F, t697: F, t111: F, t7002: F) -> (F, F, F, F, F, F) {
    let t82417 = t344 * t1049 * t225;
    let t82431 = t1926 * t221 * t10189;
    let t82502 = t6733 * t1921;
    let t82573 = t6712 * t23383;
    let t82631 = t221 * t697;
    let t82632 = t1926 * t82631;
    let t83980 = t7002 * t111;
    (t82417, t82431, t82502, t82573, t82632, t83980)
}
