//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1301/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1301<F: Float>(t4645: F, t61873: F, t640: F, t61877: F, t1333: F, t3532: F, t18397: F, t18394: F, t4669: F, t13541: F, t5527: F, t1659: F, t4397: F) -> (F, F, F, F, F, F, F) {
    let t68872 = t61873 * t4645;
    let t68874 = t4645 * t640;
    let t68875 = t61877 * t68874;
    let t68877 = t1333 * t3532;
    let t68878 = t18397 * t68877;
    let t68880 = t18394 * t4669;
    let t68882 = t4669 * t640;
    let t68883 = t18397 * t68882;
    let t68885 = t5527 * t13541;
    let t68950 = t4397 * t1659;
    (t68872, t68875, t68878, t68880, t68883, t68885, t68950)
}
