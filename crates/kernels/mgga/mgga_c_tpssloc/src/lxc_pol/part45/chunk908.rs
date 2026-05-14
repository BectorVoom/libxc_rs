//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 908/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk908<F: Float>(t31551: F, t81159: F, t115352: F, t6897: F, t6907: F, t3886: F, t7213: F, t1385: F, t1992: F, t22635: F, t225: F, t31585: F, t114264: F, t114270: F, t114279: F, t114288: F, t114292: F, t12033: F, t1375: F, t1386: F, t22670: F, t24088: F, t24092: F, t24095: F, t31564: F, t3752: F, t3882: F, t3887: F, t3911: F, t568: F, t6958: F, t6963: F, t7199: F, t8617: F, t8636: F, t8637: F) -> (F,) {
    let t115596 = t81159 * t31551;
    let t115601 = t6897 * t115352 * t6907;
    let t115614 = t3886 * t7213;
    let t115617 = t1992 * t22635 * t115614 * t1385;
    let t115619 = t31585 * t225;
    let t115622 = t3752 * t8617 * t568 + 4.0 * t3882 * t31564 - 0.76763589786250567036e-1 * t115596 + t114264 + t114270 - t114279 + 4.0 * t24095 * t6963 + t114288 + 0.82246703342411321824e-2 * t115601 + 2.0 * t6958 * t24088 + 2.0 * t1375 * t3887 * t8636 * t3911 + 4.0 * t22670 * t7199 - t12033 * t8637 - 6.0 * t6958 * t24092 + t114292 + 0.3289868133696452873e-1 * t115617 - 2.0 * t115619 * t1386;
    (t115622,)
}
