//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1150/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1150<F: Float>(t508: F, t5753: F, t5709: F, t1760: F, t3202: F, t9895: F, t1778: F, t5706: F, t5758: F, t38: F, t7679: F, t234: F, t2045: F, t76: F, t1976: F, t582: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18289 = t508 * t5753;
    let t18290 = t18289 * t5709;
    let t18292 = 6.0 * t1760 * t18290;
    let t18295 = t9895 * t3202;
    let t18296 = t1778 * t18295;
    let t18298 = 2.0 * t1760 * t18296;
    let t18304 = 2.0 * t5706 * t5758;
    let t18305 = t7679 * t38;
    let t18322 = 88.0 / 9.0 * t234;
    let t18331 = t76 * t2045;
    let t18338 = t1976 * t582;
    (t18289, t18290, t18292, t18295, t18296, t18298, t18304, t18305, t18322, t18331, t18338)
}
