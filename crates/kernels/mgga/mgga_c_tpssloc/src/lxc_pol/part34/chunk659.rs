//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 659/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk659<F: Float>(t1825: F, t7208: F, t553: F, t7918: F, t1336: F, t1814: F, t2089: F, t544: F, t7202: F, t7204: F, t7734: F, t7738: F, t7742: F, t1378: F, t1375: F, t1843: F, t2092: F, t5215: F, t5321: F, t568: F, t7174: F, t7176: F, t7194: F, t7693: F, t7698: F, t7702: F, t7910: F, t7919: F, t7925: F) -> (F, F, F, F, F) {
    let t7932 = t7208 * t1825;
    let t7934 = t553 * t7918;
    let t7936 = -t7202 - 0.3289868133696452873e-1 * t7734 - t7204 - 0.16449340668482264365e-1 * t7738 + 0.16449340668482264365e-1 * t7742 + t1814 * t2089 - t1336 * t7932 + t544 * t7934;
    let t7937 = t1378 * t7936;
    let t7939 = -t7174 - 0.3289868133696452873e-1 * t7693 - t7176 + 0.16449340668482264365e-1 * t7698 - 0.16449340668482264365e-1 * t7702 + t7910 * t568 + t7919 * t568 - t7194 * t1843 - t5215 * t2092 - t5321 * t2092 + 2.0 * t1375 * t7925 - t1375 * t7937;
    (t7932, t7934, t7936, t7937, t7939)
}
