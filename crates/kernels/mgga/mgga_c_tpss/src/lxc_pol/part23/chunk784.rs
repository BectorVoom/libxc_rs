//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 784/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk784<F: Float>(t219: F, t4434: F, t4443: F, t516: F, t73: F, t1246: F, t1625: F, t1206: F, t1228: F, t4397: F, t1226: F, t1229: F, t1634: F, t1636: F, t518: F) -> (F, F, F, F, F, F) {
    let t4445 = (t4434 + t4443) * t219;
    let t4451 = t516 * t73;
    let t4452 = t1246 * t1625;
    let t4453 = t4452 * t1206;
    let t4456 = t1228 * t4397;
    let t4459 = 3.0 * t1226 * t1636 + 3.0 * t1229 * t1634 - t4445 * t518 - 12.0 * t4451 * t4453 + 3.0 * t4456 * t516;
    (t4445, t4451, t4452, t4453, t4456, t4459)
}
