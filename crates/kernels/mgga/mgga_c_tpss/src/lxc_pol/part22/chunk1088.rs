//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1088/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1088<F: Float>(t10514: F, t18246: F, t1006: F, t750: F, t2133: F, t33: F, t2433: F, t821: F, t2428: F, t3202: F, t9895: F, t38: F, t7679: F, t2045: F, t76: F, t1976: F, t582: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18247 = t18246 * t10514;
    let t18250 = t1006 * t750;
    let t18254 = t33 * t2133;
    let t18265 = t33 * t2433;
    let t18268 = t1006 * t821;
    let t18271 = t33 * t2428;
    let t18295 = t9895 * t3202;
    let t18305 = t7679 * t38;
    let t18331 = t76 * t2045;
    let t18338 = t1976 * t582;
    (t18247, t18250, t18254, t18265, t18268, t18271, t18295, t18305, t18331, t18338)
}
