//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1157/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1157<F: Float>(t13133: F, t1689: F, t13554: F, t3493: F, t5522: F, t6076: F, t619: F, t77: F, t1317: F, t1679: F, t1678: F, t1290: F, t1981: F, t10289: F, t38: F, t1289: F, t18314: F, t18317: F, t18322: F, t3426: F, t3431: F, t5497: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19336 = 2.0 * t13133 * t1689;
    let t19338 = 2.0 * t13554 * t1689;
    let t19340 = 2.0 * t3493 * t5522;
    let t19342 = t77 * t6076 * t619;
    let t19345 = t1679 * t1317;
    let t19346 = t1678 * t19345;
    let t19349 = t1981 * t1290;
    let t19352 = t10289 * t38;
    let t19367 = -20.0 / 9.0 * t18314 * t1289 + 5.0 / 18.0 * t18317 * t3426 + 5.0 / 6.0 * t5497 * t3431 - t18322;
    (t19336, t19338, t19340, t19342, t19345, t19346, t19349, t19352, t19367)
}
