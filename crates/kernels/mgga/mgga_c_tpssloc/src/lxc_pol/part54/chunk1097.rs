//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1097/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1097<F: Float>(t1378: F, t33293: F, t6906: F, t7936: F, t6889: F, t1985: F, t2015: F, t3887: F, t31611: F, t7691: F, t6888: F, t7700: F, t1842: F, t8636: F, t2091: F, t7749: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33294 = t1378 * t33293;
    let t33296 = t6906 * t7936;
    let t33297 = t6889 * t33296;
    let t33298 = t1985 * t33297;
    let t33300 = t7936 * t2015;
    let t33301 = t3887 * t33300;
    let t33307 = t31611 * t7691;
    let t33308 = t6888 * t33307;
    let t33310 = t31611 * t7700;
    let t33311 = t1985 * t33310;
    let t33315 = t8636 * t1842;
    let t33316 = t3887 * t33315;
    let t33320 = t3887 * t2091 * t7749;
    (t33294, t33296, t33297, t33298, t33301, t33307, t33308, t33310, t33311, t33316, t33320)
}
