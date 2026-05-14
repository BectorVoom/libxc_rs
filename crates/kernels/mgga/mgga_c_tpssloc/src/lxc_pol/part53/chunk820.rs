//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 820/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk820<F: Float>(t33825: F, t33852: F, t533: F, t1390: F, t2075: F, t7801: F, t2039: F, t7890: F, t1442: F, t1459: F, t1983: F, t2040: F, t27188: F, t32235: F, t32674: F, t32676: F, t32679: F, t33234: F, t33790: F, t33793: F, t4028: F, t652: F, t7042: F, t7458: F, t7685: F, t7796: F, t7802: F, t7806: F, t7943: F, t8607: F, t8721: F, t8774: F, t8805: F) -> (F, F, F, F, F, F) {
    let t33853 = t33825 + t33852;
    let t33854 = t533 * t33853;
    let t33855 = t33854 * t1390;
    let t33857 = t2075 * t7801;
    let t33874 = t7890 * t2039;
    let t33877 = -t1442 * t8774 - 2.0 * t1459 * t32235 - 3.0 * t1983 * t33790 - t1983 * t33793 + t1983 * t33855 - 4.0 * t2040 * t27188 - 4.0 * t2040 * t33234 - 4.0 * t33857 * t652 - 4.0 * t33874 * t652 - 4.0 * t4028 * t8721 - 4.0 * t7042 * t7796 - 4.0 * t7042 * t7802 - 4.0 * t7042 * t7806 - 4.0 * t7458 * t8721 + t7685 * t8805 - 2.0 * t7943 * t8607 - t32674 - t32676 - t32679;
    (t33853, t33854, t33855, t33857, t33874, t33877)
}
