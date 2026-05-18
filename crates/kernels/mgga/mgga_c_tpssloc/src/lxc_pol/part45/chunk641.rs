//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 641/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk641<F: Float>(t1378: F, t7213: F, t1375: F, t1386: F, t2092: F, t3758: F, t3882: F, t568: F, t6893: F, t6904: F, t6909: F, t7174: F, t7176: F, t7179: F, t7192: F, t7194: F, t7199: F) -> (F, F) {
    let t7214 = t1378 * t7213;
    let t7216 = -t7174 - F::new(0.3289868133696452873e-1) * t6893 - t7176 + F::new(0.16449340668482264365e-1) * t6904 - F::new(0.16449340668482264365e-1) * t6909 + t7179 * t568 + t7192 * t568 - t7194 * t1386 - t3758 * t2092 - t3882 * t2092 + F::new(2.0) * t1375 * t7199 - t1375 * t7214;
    (t7214, t7216)
}
