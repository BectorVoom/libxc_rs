//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1018/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1018<F: Float>(t225: F, t25791: F, t1921: F, t7577: F, t25820: F, t23328: F, t23394: F, t4657: F, t6703: F, t25789: F, t25822: F, t28: F, t40772: F, t111: F, t26097: F, t1834: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t88145 = t25791 * t225;
    let t88162 = t7577 * t1921;
    let t88744 = t25820 * t225;
    let t88772 = t23328 * t23394;
    let t89598 = t6703 * t4657;
    let t89620 = t25789 * t225;
    let t89666 = t25822 * t225;
    let t89953 = t40772 * t28;
    let t90400 = t26097 * t111;
    let t90544 = t794 * t1834;
    (t88145, t88162, t88744, t88772, t89598, t89620, t89666, t89953, t90400, t90544)
}
