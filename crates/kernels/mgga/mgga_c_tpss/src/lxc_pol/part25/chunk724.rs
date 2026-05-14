//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 724/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk724<F: Float>(t5036: F, t981: F, t1483: F, t373: F, t3990: F, t5013: F, t5018: F, t978: F, t1485: F, t198: F, t2814: F, t330: F, t4840: F, t4842: F, t4846: F, t4878: F, t4881: F, t4947: F, t4949: F, t4951: F, t4955: F, t4959: F, t4963: F, t995: F) -> (F, F, F, F) {
    let t5037 = t981 * t5036;
    let t5039 = -2.0 * t1483 * t3990 + t373 * t5013 + 2.0 * t5018 * t978 - t5037 * t978;
    let t5043 = t1485 * t1485;
    let t5047 = -t198 * t2814 * t330 * t5043 + t198 * t330 * t5039 * t995 - t4840 + t4842 - t4846 + t4878 + t4881 + t4947 + t4949 - t4951 + t4955 - t4959 - t4963;
    (t5037, t5039, t5043, t5047)
}
