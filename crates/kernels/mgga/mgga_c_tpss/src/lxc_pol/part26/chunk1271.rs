//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1271/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1271<F: Float>(t20854: F, t219: F, t2712: F, t9738: F, t1107: F, t1883: F, t2710: F, t20853: F, t20862: F, t5570: F, t8547: F, t28778: F, t3048: F, t19142: F, t6513: F, t12404: F, t6013: F) -> (F, F, F, F, F, F, F, F) {
    let t68192 = t20854 * t219;
    let t68222 = t2712 * t9738;
    let t68224 = t1883 * t2710 * t68222 * t1107;
    let t68235 = t1107 * t20853;
    let t68273 = t20862 * t5570;
    let t68276 = t1883 * t8547;
    let t68278 = t68276 * t28778 * t3048;
    let t68321 = t68276 * t28778 * t1107;
    let t68356 = t6513 * t19142;
    let t68361 = 5.0 / 5184.0 * t6013 * t12404;
    (t68192, t68224, t68235, t68273, t68278, t68321, t68356, t68361)
}
