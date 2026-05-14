//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 698/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk698<F: Float>(t24115: F, t24137: F, t1378: F, t1323: F, t7191: F, t1385: F, t7213: F, t3887: F, t22923: F, t22925: F, t2085: F, t3752: F, t1375: F, t22664: F, t22668: F, t22676: F, t22688: F, t22907: F, t22909: F, t22918: F, t22921: F, t22928: F, t22931: F, t22936: F, t22940: F, t568: F) -> (F, F, F, F, F, F) {
    let t24138 = t24115 + t24137;
    let t24139 = t1378 * t24138;
    let t24141 = t1323 * t7191;
    let t24146 = t7213 * t1385;
    let t24147 = t3887 * t24146;
    let t24156 = 0.12793931631041761173e0 * t22923;
    let t24157 = 0.52089578783527170489e-1 * t22925;
    let t24162 = t3752 * t2085;
    let t24164 = -t1375 * t24139 + 2.0 * t24141 * t568 - 0.16449340668482264365e-1 * t22664 - 0.3289868133696452873e-1 * t22668 + 4.0 * t1375 * t24147 + 0.16449340668482264365e-1 * t22676 + 0.9869604401089358619e-1 * t22688 + 0.15352717957250113407e0 * t22907 + 0.76763589786250567036e-1 * t22909 - 0.3289868133696452873e-1 * t22918 + 0.3289868133696452873e-1 * t22921 + t24156 + t24157 - 0.16449340668482264365e-1 * t22928 - 0.6579736267392905746e-1 * t22931 + 0.3289868133696452873e-1 * t22936 - 0.76763589786250567036e-1 * t22940 + t24162 * t568;
    (t24138, t24139, t24141, t24147, t24162, t24164)
}
