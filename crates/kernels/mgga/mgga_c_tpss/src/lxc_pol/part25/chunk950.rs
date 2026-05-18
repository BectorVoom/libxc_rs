//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 950/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk950<F: Float>(t11942: F, t3001: F, t4180: F, t11875: F, t1505: F, t2861: F, t1053: F, t4117: F, t1523: F, t2954: F, t926: F, t9637: F) -> (F, F, F, F, F, F, F, F) {
    let t12146 = F::new(0.11415555555555555555e-1) * t11942;
    let t12210 = t4180 * t3001;
    let t12231 = F::new(0.23744444444444444444e-1) * t11875;
    let t12232 = F::new(0.11872222222222222222e-1) * t11942;
    let t12244 = t1505 * t2861;
    let t12264 = t4117 * t1053;
    let t12269 = t1523 * t2954;
    let t12278 = t926 * t9637;
    (t12146, t12210, t12231, t12232, t12244, t12264, t12269, t12278)
}
