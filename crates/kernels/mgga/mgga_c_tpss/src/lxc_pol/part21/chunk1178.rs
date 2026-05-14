//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1178/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1178<F: Float>(t38: F, t7679: F, t48: F, t588: F, t2009: F, t43: F, t234: F, t1985: F, t1992: F, t2003: F, t49: F, t5497: F, t581: F, t72: F, t1679: F, t5502: F, t5506: F) -> (F, F, F, F, F, F, F, F) {
    let t18305 = t7679 * t38;
    let t18314 = t588 * t48;
    let t18317 = t43 * t2009;
    let t18322 = 88.0 / 9.0 * t234;
    let t18323 = 88.0 / 9.0 * t2003 * t49 - 40.0 / 9.0 * t18314 * t581 + 5.0 / 18.0 * t18317 * t1985 + 5.0 / 6.0 * t5497 * t1992 - t18322;
    let t18324 = t18323 * t72;
    let t18325 = t18324 * t1679;
    let t18328 = t5502 * t5506;
    (t18305, t18314, t18317, t18322, t18323, t18324, t18325, t18328)
}
