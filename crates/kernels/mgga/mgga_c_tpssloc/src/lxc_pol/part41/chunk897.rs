//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 897/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk897<F: Float>(t2696: F, t4166: F, t849: F, t13176: F, t842: F, t1516: F, t9601: F, t68: F, t9971: F, t226: F, t4265: F, t814: F, t225: F, t4149: F, t4351: F, t892: F) -> (F, F, F, F, F, F, F, F) {
    let t13360 = t4166 * t2696;
    let t13362 = 7.0 / 576.0 * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13433 = t814 * t4265;
    let t13463 = t4149 * t225;
    let t13515 = t4351 * t892;
    (t13360, t13362, t13365, t13368, t13397, t13433, t13463, t13515)
}
