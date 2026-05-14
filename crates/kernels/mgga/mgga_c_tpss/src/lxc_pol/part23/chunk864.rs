//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 864/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk864<F: Float>(t1398: F, t30: F, t1288: F, t1692: F, t1713: F, t2439: F, t5590: F, t6121: F, t6149: F, t1364: F, t207: F, t6148: F, t198: F, t823: F, t33: F, t1497: F) -> (F, F, F, F, F, F, F, F) {
    let t6153 = t30 * t1398;
    let t6160 = 3.0 / 2.0 * t2439 * t6121 + t1692 * t6149 * t30 / 2.0 - t1692 * t5590 * t6153 / 2.0 + t1692 * t1713 * t1288 / 2.0;
    let t6192 = t1713 * t1364;
    let t6195 = t207 * t6148;
    let t6200 = -t1398 * t1692 * t5590 + t198 * t6195 * t823 + 3.0 * t2439 * t6192;
    let t6207 = t33 * t1364;
    let t6208 = t1713 * t6207;
    let t6214 = t33 * t1398;
    let t6221 = 3.0 / 2.0 * t2439 * t6208 + t1692 * t6149 * t33 / 2.0 - t1692 * t5590 * t6214 / 2.0 + t1692 * t1713 * t1497 / 2.0;
    (t6153, t6160, t6192, t6200, t6207, t6208, t6214, t6221)
}
