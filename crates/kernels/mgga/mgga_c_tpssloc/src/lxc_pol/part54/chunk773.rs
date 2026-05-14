//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 773/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk773<F: Float>(t1375: F, t1843: F, t2092: F, t5215: F, t5321: F, t568: F, t7174: F, t7176: F, t7194: F, t7693: F, t7698: F, t7702: F, t7910: F, t7919: F, t7925: F, t7937: F) -> (F,) {
    let t7939 = -t7174 - 0.3289868133696452873e-1 * t7693 - t7176 + 0.16449340668482264365e-1 * t7698 - 0.16449340668482264365e-1 * t7702 + t7910 * t568 + t7919 * t568 - t7194 * t1843 - t5215 * t2092 - t5321 * t2092 + 2.0 * t1375 * t7925 - t1375 * t7937;
    (t7939,)
}
