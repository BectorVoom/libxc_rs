//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 894/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk894<F: Float>(t1992: F, t31559: F, t80650: F, t115331: F, t115334: F, t115337: F, t115339: F, t115341: F, t115354: F, t12030: F, t12444: F, t2016: F, t2092: F, t22629: F, t22652: F, t22912: F, t26224: F, t26989: F, t31655: F, t80699: F, t80704: F, t81319: F, t84700: F, t8637: F, t91505: F, t93319: F) -> (F,) {
    let t115359 = t1992 * t80650 * t31559;
    let t115364 = -2.0 * t80699 * t2092 + 24.0 * t26224 * t93319 * t22629 - t115331 - 0.3289868133696452873e-1 * t115334 - 0.16449340668482264365e-1 * t115337 + 0.76763589786250567036e-1 * t115339 + 0.38381794893125283518e-1 * t115341 - t84700 * t2016 - 12.0 * t26224 * t26989 * t22652 - 12.0 * t91505 * t31655 - 6.0 * t26224 * t26989 * t22912 + 0.16449340668482264365e-1 * t115354 - t81319 * t2092 - t80704 * t2092 + 0.3289868133696452873e-1 * t115359 - t12030 * t8637 - 2.0 * t12444 * t8637;
    (t115364,)
}
