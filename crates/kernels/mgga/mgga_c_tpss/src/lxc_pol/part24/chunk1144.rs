//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1144/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1144<F: Float>(t508: F, t5753: F, t48: F, t588: F, t2009: F, t43: F, t234: F, t5486: F, t7682: F, t38: F, t5501: F, t1981: F, t7690: F) -> (F, F, F, F, F, F, F, F) {
    let t18289 = t508 * t5753;
    let t18314 = t588 * t48;
    let t18317 = t43 * t2009;
    let t18322 = 88.0 / 9.0 * t234;
    let t18335 = t7682 * t5486;
    let t18341 = t38 * t5501;
    let t18342 = t1981 * t18341;
    let t18345 = t7690 * t5486;
    (t18289, t18314, t18317, t18322, t18335, t18341, t18342, t18345)
}
