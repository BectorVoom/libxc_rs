//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1414/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1414<F: Float>(t225: F, t3755: F, t3700: F, t570: F, t1390: F, t3914: F, t3698: F, t3701: F, t112: F, t3931: F, t111: F, t1395: F) -> (F, F, F, F, F, F) {
    let t12444 = t3755 * t225;
    let t12461 = F::new(1.0) / t3700 / t570;
    let t12466 = t3914 * t1390;
    let t12477 = t3698 * t3701;
    let t12521 = t3931 * t112;
    let t12524 = t1395 * t111;
    (t12444, t12461, t12466, t12477, t12521, t12524)
}
