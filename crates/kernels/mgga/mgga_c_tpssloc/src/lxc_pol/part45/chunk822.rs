//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 822/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk822<F: Float>(t1338: F, t7191: F, t1352: F, t24063: F, t553: F, t2085: F, t3787: F, t3793: F, t3856: F, t7208: F, t1336: F, t22735: F, t22743: F, t22745: F, t22749: F, t22752: F, t22884: F, t22888: F, t22895: F, t22900: F, t544: F) -> F {
    let t24116 = t1338 * t7191;
    let t24117 = t24116 * t1352;
    let t24121 = t553 * t24063;
    let t24127 = t3787 * t2085;
    let t24128 = t24127 * t3793;
    let t24131 = t7208 * t3856;
    let t24137 = -F::new(2.0) * t1336 * t24117 + F::new(0.6579736267392905746e-1) * t22735 + t544 * t24121 - F::new(0.16449340668482264365e-1) * t22743 + F::new(0.76763589786250567036e-1) * t22745 + F::new(0.9869604401089358619e-1) * t22749 + F::new(0.15352717957250113407e0) * t22752 + F::new(2.0) * t1336 * t24128 - t1336 * t24131 - F::new(0.6579736267392905746e-1) * t22884 - F::new(0.3289868133696452873e-1) * t22888 + F::new(0.3289868133696452873e-1) * t22895 + F::new(0.3289868133696452873e-1) * t22900;
    t24137
}
