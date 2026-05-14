//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1194/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1194<F: Float>(t1659: F, t4519: F, t1270: F, t13671: F, t1268: F, t5458: F, t1206: F, t21011: F, t19619: F, t6242: F, t4645: F, t61873: F, t640: F, t61877: F, t1333: F, t3532: F) -> (F, F, F, F, F, F, F, F) {
    let t68798 = t1659 * t4519;
    let t68823 = t1270 * t13671;
    let t68827 = t5458 * t1268;
    let t68838 = t21011 * t1206;
    let t68868 = t6242 * t19619;
    let t68872 = t61873 * t4645;
    let t68874 = t4645 * t640;
    let t68875 = t61877 * t68874;
    let t68877 = t1333 * t3532;
    (t68798, t68823, t68827, t68838, t68868, t68872, t68875, t68877)
}
