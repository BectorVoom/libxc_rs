//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1251/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1251<F: Float>(t1006: F, t2436: F, t18546: F, t5705: F, t112: F, t789: F, t234: F, t630: F, t640: F, t2073: F, t599: F, t2074: F, t18394: F, t2100: F, t68: F, t7594: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61703 = t2436 * t1006;
    let t61801 = t5705 * t18546;
    let t61868 = t789 * t112;
    let t61869 = 154.0 / 27.0 * t61868;
    let t61870 = t234 * t630;
    let t61871 = t61870 * t640;
    let t61873 = t599 * t2073;
    let t61874 = t61873 * t2074;
    let t61876 = t18394 * t2100;
    let t61877 = t68 * t7594;
    (t61703, t61801, t61869, t61870, t61871, t61873, t61874, t61876, t61877)
}
