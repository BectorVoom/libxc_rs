//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1051/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1051<F: Float>(t24115: F, t24137: F, t1378: F, t1323: F, t7191: F, t1385: F, t7213: F, t3887: F, t22923: F, t22925: F, t2085: F, t3752: F) -> (F, F, F, F, F, F, F) {
    let t24138 = t24115 + t24137;
    let t24139 = t1378 * t24138;
    let t24141 = t1323 * t7191;
    let t24146 = t7213 * t1385;
    let t24147 = t3887 * t24146;
    let t24156 = F::new(0.12793931631041761173e0) * t22923;
    let t24157 = F::new(0.52089578783527170489e-1) * t22925;
    let t24162 = t3752 * t2085;
    (t24138, t24139, t24141, t24147, t24156, t24157, t24162)
}
